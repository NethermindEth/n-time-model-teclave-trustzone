// Step 1: Header translation as Rust skeleton

#![no_std]
extern crate alloc;

use optee_utee::{AlgorithmId, Asymmetric, Attribute, AttributeId, AttributeMemref, DataFlag, Digest, Error, ErrorKind, ObjHandle, ObjectHandle, ObjectStorageConstants, OperationMode, PersistentObject, Result, TransientObject, TransientObjectType};
use optee_utee_sys::{TEE_Attribute, TEE_FreeTransientObject, TEE_PopulateTransientObject};

// Token structure
#[repr(C)]
pub struct RestrictToken {
    pub sequence_number: u32,
    pub n: u32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct RestrictState {
    pub last_used_sequence_number: u32,
    pub remaining_invocations: u32,
}

// Library handle
pub struct RestrictHandle {
    pub pub_key: TransientObject,
}

/// Initialize the restriction library.
pub fn restrict_init(
    handle: &mut RestrictHandle,
    modulus: &[u8],
    exponent: &[u8],
) -> Result<()> {
    // Step 1: Build attributes for the RSA public key
    let attrs = make_rsa_pubkey_attrs(modulus, exponent);

    // Step 2: Allocate a transient object for the RSA public key
    let mut pub_key = TransientObject::allocate(TransientObjectType::RsaPublicKey, modulus.len() * 8)?;

    // Step 3: Populate the object with the modulus and exponent
    pub_key.populate(&attrs)?;

    // Step 4: Save the public key object in the handle
    handle.pub_key = pub_key;

    Ok(())
}

static STATE_OBJECT_NAME: &str = "invocation_state";

pub fn get_or_create_state() -> Result<(PersistentObject, RestrictState)> {
    match PersistentObject::open(
        ObjectStorageConstants::Private,
        STATE_OBJECT_NAME.as_ref(),
        DataFlag::ACCESS_READ | DataFlag::ACCESS_WRITE,
    ) {
        Ok(mut obj) => {
            let mut buf = RestrictState::default();
            let buf_slice = unsafe {
                core::slice::from_raw_parts_mut(
                    (&mut buf as *mut _) as *mut u8,
                    core::mem::size_of::<RestrictState>(),
                )
            };
            let bytes_read = obj.read(buf_slice)?;
            if bytes_read as usize != core::mem::size_of::<RestrictState>() {
                return Err(Error::from(ErrorKind::Generic));
            }
            Ok((obj, buf))
        }
        Err(e) if e.kind() == ErrorKind::ItemNotFound => {
            let initial = RestrictState::default();
            let obj = PersistentObject::create(
                ObjectStorageConstants::Private,
                STATE_OBJECT_NAME.as_ref(),
                DataFlag::ACCESS_WRITE,
                None,
                unsafe {
                    core::slice::from_raw_parts(
                        (&initial as *const _) as *const u8,
                        core::mem::size_of::<RestrictState>(),
                    )
                },
            )?;
            Ok((obj, initial))
        }
        Err(e) => Err(e),
    }
}


pub fn update_state(mut obj: PersistentObject, state: &RestrictState) -> Result<()> {
    use optee_utee::Whence;

    // Seek to the beginning of the object
    obj.seek(0, Whence::DataSeekEnd)?;

    // Write the updated state
    let buf = unsafe {
        core::slice::from_raw_parts(
            (state as *const _) as *const u8,
            core::mem::size_of::<RestrictState>(),
        )
    };

    obj.write(buf)?;
    Ok(())
}



fn make_rsa_pubkey_attrs<'a>(
    modulus: &'a [u8],
    exponent: &'a [u8],
) -> [Attribute; 2] {
    [
        AttributeMemref::from_ref(AttributeId::RsaModulus, modulus).into(),
        AttributeMemref::from_ref(AttributeId::RsaPublicExponent, exponent).into(),
    ]
}

/// Submit a token to grant invocations.

pub fn restrict_submit_token(
    handle: &RestrictHandle,
    token: &RestrictToken,
    signature: &[u8],
) -> Result<()> {
    if signature.len() != 256 {
        return Err(Error::from(ErrorKind::BadParameters));
    }

    // Step 1: Hash the token (SHA-256)
    let token_bytes = unsafe {
        core::slice::from_raw_parts(
            token as *const RestrictToken as *const u8,
            core::mem::size_of::<RestrictToken>(),
        )
    };
    let mut hash = [0u8; 32];
    let mut hasher = Digest::allocate(AlgorithmId::Sha256)?;
    hasher.do_final(token_bytes, &mut hash)?;

    // Step 2: Verify signature (RSA PKCS1v1.5 + SHA-256)
    let mut verifier = Asymmetric::allocate(
        AlgorithmId::RsassaPkcs1V15Sha256,
        OperationMode::Verify,
        2048,
    )?;
    verifier.set_key(&handle.pub_key)?;
    verifier
        .verify_digest(&[], &hash, signature)
        .map_err(|_| Error::from(ErrorKind::Security))?;

    // Step 3: Load state or create new
    let (mut state_obj, mut state) = get_or_create_state()?;

    // Step 4: Replay protection
    if token.sequence_number <= state.last_used_sequence_number {
        return Err(Error::from(ErrorKind::BadParameters));
    }

    // Step 5: Update state
    state.last_used_sequence_number = token.sequence_number;
    state.remaining_invocations = state
        .remaining_invocations
        .saturating_add(token.n);

    update_state(state_obj, &state)
}



/// Check if an invocation is allowed and decrement the count if so.
pub fn restrict_check_invocation(_handle: &RestrictHandle) -> Result<()> {
    todo!("Implement restrict_check_invocation");
}

/// Cleanup the library resources.
pub fn restrict_cleanup(handle: &mut RestrictHandle) {
    // Reset to a null TransientObject to release the key material early
    handle.pub_key = TransientObject::null_object();
}