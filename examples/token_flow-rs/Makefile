CROSS_COMPILE_HOST ?= aarch64-linux-gnu-
CROSS_COMPILE_TA ?= aarch64-linux-gnu-
TARGET_HOST ?= aarch64-unknown-linux-gnu
TARGET_TA ?= aarch64-unknown-linux-gnu

.PHONY: host ta

all: host ta

host:
	$(q)make -C host TARGET=$(TARGET_HOST) \
		CROSS_COMPILE=$(CROSS_COMPILE_HOST)

ta:
	$(q)make -C ta TARGET=$(TARGET_TA) \
		CROSS_COMPILE=$(CROSS_COMPILE_TA)

clean:
	$(q)make -C host clean
	$(q)make -C ta clean
