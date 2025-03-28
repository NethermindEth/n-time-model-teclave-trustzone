NAME := token_flow-rs

TARGET ?= aarch64-unknown-linux-gnu
CROSS_COMPILE ?= aarch64-linux-gnu-
OBJCOPY := $(CROSS_COMPILE)objcopy
LINKER_CFG := target.$(TARGET).linker=\"$(CROSS_COMPILE)gcc\"

OUT_DIR := $(CURDIR)/target/$(TARGET)/release


all: host strip

host:
	@cargo build --target $(TARGET_HOST) --release --config $(LINKER_CFG)

strip: host
	@$(OBJCOPY) --strip-unneeded $(OUT_DIR)/$(NAME) $(OUT_DIR)/$(NAME)

clean:
	@cargo clean
