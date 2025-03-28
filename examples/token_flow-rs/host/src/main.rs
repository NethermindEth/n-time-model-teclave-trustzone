use std::fs::File;
use std::io::Read;
use optee_teec::{Context, ErrorKind, Operation, Session, Uuid, Param, ParamTmpRef};
use proto::UUID;

use clap::{Parser, Subcommand};

/// Interact with the TA using `submit_token` or `invoke` commands.
#[derive(Parser)]
#[command(name = "ta-runner")]
#[command(about = "Interact with the TA via OP-TEE client commands", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Submit a token and its signature to the TA.
    SubmitToken {
        /// Path to the token file (e.g., token.bin)
        token_path: String,

        /// Path to the signature file (e.g., sig.bin)
        sig_path: String,
    },

    /// Invoke a proprietary TA function with an integer and a string.
    Invoke {
        /// Integer value to pass to the TA
        int_value: i32,

        /// String value to pass to the TA
        string_value: String,
    },
}


fn token_flow(cli: Cli, session: &mut Session) -> optee_teec::Result<()> {
    match cli.command {
        Commands::SubmitToken { token_path, sig_path } => {
            // Load token and signature from files
            let mut token = Vec::new();
            File::open(token_path)
                .and_then(|mut f| f.read_to_end(&mut token))
                .map_err(|_| ErrorKind::BadParameters)?;

            let mut signature = Vec::new();
            File::open(sig_path)
                .and_then(|mut f| f.read_to_end(&mut signature))
                .map_err(|_| ErrorKind::BadParameters)?;

            let mut operation = Operation::new(
                0,
                ParamTmpRef::new_input(&token),
                ParamTmpRef::new_input(&signature),
                ParamTmpRef::new_input(&[]),
                ParamTmpRef::new_input(&[]),
            );
            session.invoke_command(proto::Command::SubmitToken.into(), &mut operation)?;
            println!("Token submitted successfully");
        }

        Commands::Invoke { int_value, string_value } => {
            let mut input = Vec::new();
            input.extend_from_slice(&int_value.to_ne_bytes());
            input.extend_from_slice(string_value.as_bytes());
            input.push(0); // null terminator

            let mut result = [0u8; std::mem::size_of::<i32>()];

            let mut operation = Operation::new(
                0,
                ParamTmpRef::new_input(&input),
                ParamTmpRef::new_output(&mut result),
                ParamTmpRef::new_input(&[]),
                ParamTmpRef::new_input(&[]),
            );

            session.invoke_command(proto::Command::InvokeProprietary.into(), &mut operation)?;

            let result_value = i32::from_ne_bytes(result);
            println!("Function invoked, result: {}", result_value);
        }
    }

    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let cli = Cli::parse();
    let mut ctx = Context::new().unwrap();

    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    token_flow(cli, &mut session)?;

    println!("Success");
    Ok(())
}
