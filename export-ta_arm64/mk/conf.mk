sm := ta_arm64
sm-ta_arm64 := y
CFG_ARM64_ta_arm64 := y
ta_arm64-platform-cppflags := -DARM64=1 -D__LP64__=1 -DMBEDTLS_SELF_TEST
ta_arm64-platform-cflags := -Os -g3 -fpic -mstrict-align -mno-outline-atomics -mbranch-protection=pac-ret+leaf -fstack-protector-strong
ta_arm64-platform-aflags := -pipe -g
ta_arm64-platform-cxxflags := -fpic -Os -g3
CFG_TA_FLOAT_SUPPORT := y
CFG_SECURE_DATA_PATH := n
CFG_TA_MBEDTLS_SELF_TEST := y
CFG_TA_MBEDTLS := y
CFG_TA_MBEDTLS_MPI := y
CFG_SYSTEM_PTA := y
CFG_FTRACE_SUPPORT := n
CFG_UNWIND := y
CFG_TA_BTI := n
CFG_TA_PAUTH := y
CFG_CORE_TPM_EVENT_LOG := n
CFG_TA_BGET_TEST := n
CFG_ATTESTATION_PTA := n
CFG_MEMTAG := n
CFG_TA_MBEDTLS := y
CROSS_COMPILE64 ?= $(CROSS_COMPILE)
CROSS_COMPILE_ta_arm64 ?= $(CROSS_COMPILE64)
COMPILER ?= gcc
COMPILER_ta_arm64 ?= $(COMPILER)
PYTHON3 ?= python3
CFG_TEE_TA_LOG_LEVEL ?= 1

