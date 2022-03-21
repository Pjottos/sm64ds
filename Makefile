define validate-option
  # value must be part of the list
  ifeq ($$(filter $($(1)),$(2)),)
    $$(error Value of $(1) must be one of the following: $(2))
  endif
  # value must be a single word (no whitespace)
  ifneq ($$(words $($(1))),1)
    $$(error Value of $(1) must be one of the following: $(2))
  endif
endef

VERSION ?= us
$(eval $(call validate-option,VERSION,us))

VERBOSE ?= 0
ifeq ($(VERBOSE), 0)
    V := @
endif

CC := clang
LD := ld.lld
OBJCOPY := llvm-objcopy

CFLAGS_ARM9 := -target armv5te-none-eabi

BUILD_DIR_BASE := build
BUILD_DIR := $(BUILD_DIR_BASE)/$(VERSION)

ARM9_SRC_DIRS := asm/arm9
ARM9_BOOT_SRC_DIR := asm/arm9/boot

ALL_DIRS := $(ARM9_SRC_DIRS) $(ARM9_BOOT_SRC_DIR)
DUMMY != mkdir -p $(addprefix $(BUILD_DIR)/, $(ALL_DIRS))

ARM9_BOOT_S_FILES := $(wildcard $(ARM9_BOOT_SRC_DIR)/*.S)

ARM9_BOOT_O_FILES := $(foreach file,$(ARM9_BOOT_S_FILES),$(BUILD_DIR)/$(file:.S=.o))

# Pretty print helper
PRINT := printf

NO_COL := \033[0m
RED := \033[0;31m
GREEN := \033[0;32m
BLUE := \033[0;34m
YELLOW := \033[0;33m
BLINK := \033[33;5m

define print
	@$(PRINT) "$(GREEN)$(1) $(YELLOW)$(2)$(GREEN) -> $(BLUE)$(3)$(NO_COL)\n"
endef
define print_single
	@$(PRINT) "$(GREEN)$(1) $(YELLOW)$(2)$(NO_COL)\n"
endef

# Targets
all: $(BUILD_DIR)/arm9.elf

clean:
	-rm -r $(BUILD_DIR_BASE)
	
distclean: clean
	cd util
	cargo clean

$(BUILD_DIR)/asm/arm9/%.o: asm/arm9/%.S
	$(call print,Assembling:,$<,$@)
	$(V)$(CC) $(CFLAGS_ARM9) -c $< -o $@

$(BUILD_DIR)/arm9.elf: $(ARM9_BOOT_O_FILES) arm9.ld
	$(call print_single,Linking:,$@)
	$(V)$(LD) -T arm9.ld -L $(BUILD_DIR)/$(ARM9_BOOT_SRC_DIR) -o $@
