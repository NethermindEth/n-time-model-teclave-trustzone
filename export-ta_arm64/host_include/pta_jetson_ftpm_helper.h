/* SPDX-License-Identifier: BSD-2-Clause */
/*
 * Copyright (c) 2023-2024, NVIDIA CORPORATION & AFFILIATES.
 */

#ifndef __JETSON_FTPM_HELPER_PTA_H__
#define __JETSON_FTPM_HELPER_PTA_H__

/*
 * Each trusted app UUID should have a unique UUID that is
 * generated from a UUID generator such as
 * https://www.uuidgenerator.net/
 *
 * UUID : {6c879517-2dfc-4663-863d-4896e8ccbe3a}
 */
#define FTPM_HELPER_PTA_UUID \
		{ 0x6c879517, 0x2dfc, 0x4663, \
			{0x86, 0x3d, 0x48, 0x96, 0xe8, 0xcc, 0xbe, 0x3a} }

/* Jetson fTPM helper PTA version */
#define FTPM_HELPER_PTA_VERSION_MAJOR		2
#define FTPM_HELPER_PTA_VERSION_MINOR		0

#define FTPM_HELPER_PTA_NS_STATE_NOT_READY	0xff000001
#define FTPM_HELPER_PTA_NS_STATE_READY		0xff000002
#define FTPM_HELPER_PTA_OFFLINE_PROV_MODE	0xff00000a
#define FTPM_HELPER_PTA_ONLINE_PROV_MODE	0xff00000b
#define FTPM_HELPER_PTA_UNKNOWN_PROV_MODE	0xff00000c

#define FTPM_HELPER_PTA_ECID_LENGTH		8U
#define FTPM_HELPER_PTA_SN_LENGTH		10U
/* EK Certificate buffer size */
#define FTPM_HELPER_PTA_EK_CERT_BUF_SIZE	2048U
/* EK CSR buffer size */
#define FTPM_EK_CSR_BUF_SIZE			2048U
/* EK CSR signature buffer size */
#define FTPM_EK_CSR_SIG_SIZE			80U

/*
 * FTPM_HELPER_PTA_CMD_PING_NS - Ping NS world is ready for TEE services.
 * param[0] out (value) a: normal world status
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_PING_NS		0xffff0001

/*
 * FTPM_HELPER_PTA_CMD_QUERY_SN - Query the device serial number
 * param[0] out (memref) data buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_QUERY_SN		0xffff0002

/*
 * FTPM_HELPER_PTA_CMD_QUERY_ECID - Query the device ECID
 * param[0] out (memref) data buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_QUERY_ECID		0xffff0003

/*
 * FTPM_HELPER_PTA_CMD_QUERY_PROV_MODE - Query the fTPM provisioning mode.
 * param[0] out (value) a: The defined value of provisioning mode.
 * param[1] out (value) a: The version major number.
 * param[2] out (value) a: The version major minor.
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_QUERY_PROV_MODE	0xffff0004

/*
 * FTPM_HELPER_PTA_CMD_GET_RSA_EK_CERT - Get the fTPM RSA EK Certificate
 * param[0] out (memref) data buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_GET_RSA_EK_CERT	0xffff0005

/*
 * FTPM_HELPER_PTA_CMD_GET_EC_EK_CERT - Get the fTPM EC EK Certificate
 * param[0] out (memref) data buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_GET_EC_EK_CERT	0xffff0006

/*
 * FTPM_HELPER_PTA_CMD_GET_SID_CERT - Get the Silicon ID Certificate
 * param[0] out (memref) data buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_GET_SID_CERT	0xffff0007

/*
 * FTPM_HELPER_PTA_CMD_GET_FW_ID_CERT - Get the Firmware ID Certificate
 * param[0] out (memref) data buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_GET_FW_ID_CERT	0xffff0008

/*
 * FTPM_HELPER_PTA_CMD_GET_RSA_EK_CSR - Get the fTPM RSA EK CSR
 * param[0] out (memref) data buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_GET_RSA_EK_CSR	0xffff0009

/*
 * FTPM_HELPER_PTA_CMD_GET_EC_EK_CSR - Get the fTPM EC EK CSR
 * param[0] out (memref) data buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_GET_EC_EK_CSR	0xffff000a

/*
 * FTPM_HELPER_PTA_CMD_SIGN_EK_CSR - Receive the hash of EK CSR and sign it
 * param[0] in (memref) the EK CSR digest buffer and size
 * param[1] out (memref) the EK CSR signature buffer and size
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_SIGN_EK_CSR		0xffff000b

/*
 * FTPM_HELPER_PTA_CMD_INJECT_EPS - Set the EPS explicitly from outside
 * param[0] in  (memref) eps buffer and size
 * param[1] unused
 * param[2] unused
 * param[3] unused
 */
#define FTPM_HELPER_PTA_CMD_INJECT_EPS		0xffff000c

#endif /* __JETSON_FTPM_HELPER_PTA_H__ */
