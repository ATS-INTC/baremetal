TEST ?= 
FEATURES ?= 
LOG := info
TEST_DIR = driver-test
TARGET_DIR = target/riscv64gc-unknown-none-elf
MODE := release
OPENSBI_DIR = opensbi
PLATFORM = axu15eg
OPENSBI_OBJMK = $(OPENSBI_DIR)/platform/$(PLATFORM)/objects.mk
FW_PAYLOAD = $(OPENSBI_DIR)/build/platform/$(PLATFORM)/firmware/fw_payload.bin

ELF := $(TARGET_DIR)/$(MODE)/$(TEST)
ASM := $(TARGET_DIR)/$(MODE)/$(TEST).asm
BIN := $(TARGET_DIR)/$(MODE)/$(TEST).bin


OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

build:
ifdef FEATURES
	cd $(TEST_DIR)/$(TEST) && LOG=$(LOG) cargo build --$(MODE) --features $(FEATURES)
else
	cd $(TEST_DIR)/$(TEST) && LOG=$(LOG) cargo build --$(MODE)
endif
	$(OBJCOPY) $(ELF) --strip-all -O binary $(BIN)

disasm: build
	$(OBJDUMP) -S -t $(ELF) > $(ASM)

run: build
	sed -i "/FW_PAYLOAD_PATH=/d" $(OPENSBI_OBJMK)
	echo "FW_PAYLOAD_PATH=../$(BIN)" >> $(OPENSBI_OBJMK)
	make -C opensbi PLATFORM=$(PLATFORM) CROSS_COMPILE=riscv64-unknown-linux-gnu-
	scp $(FW_PAYLOAD) axu15eg:~
	ssh axu15eg "./start_rocket.sh"

clean:
	rm -rf target