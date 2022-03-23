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

# Include dirs
INCLUDE_DIRS := include src
INC_CFLAGS := $(foreach inc,$(INCLUDE_DIRS),-I$(inc))

MATCHING ?= 1
ifeq ($(MATCHING), 1)
    CC := wine toolchain/mwccarm.exe
    CFLAGS_ARM9 := -proc v5te -opt full $(INC_CFLAGS)
else
    CC := clang
    CFLAGS_ARM9 := -target armv5te-none-eabi -O3 $(INC_CFLAGS)
endif
    
AS := clang
ASFLAGS_ARM9 := -target armv5te-none-eabi
LD := ld.lld
OBJCOPY := llvm-objcopy

# Source and object files
BUILD_DIR_BASE := build
BUILD_DIR := $(BUILD_DIR_BASE)/$(VERSION)

ARM9_ASM_DIRS := asm/arm9 asm/arm9/boot
ARM9_SRC_DIRS := src/arm9

ALL_DIRS := $(ARM9_ASM_DIRS) $(ARM9_SRC_DIRS)
DUMMY != mkdir -p $(addprefix $(BUILD_DIR)/, $(ALL_DIRS))

ARM9_S_FILES := $(foreach dir,$(ARM9_ASM_DIRS),$(wildcard $(dir)/*.S))
ARM9_CPP_FILES := $(foreach dir,$(ARM9_SRC_DIRS),$(wildcard $(dir)/*.cpp))

ARM9_O_FILES := $(foreach file,$(ARM9_CPP_FILES),$(BUILD_DIR)/$(file:.cpp=.o)) \
                $(foreach file,$(ARM9_S_FILES),$(BUILD_DIR)/$(file:.S=.o))

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
all: $(BUILD_DIR)/arm9.elf utils
	 
clean:
	$(V)-rm -r $(BUILD_DIR_BASE)
	
distclean: clean
	$(V)cd util && cargo clean

utils:
	@$(PRINT) "$(GREEN)Building utils$(NO_COL)\n"
	$(V)cd util && cargo build --release

$(BUILD_DIR)/asm/arm9/%.o: asm/arm9/%.S
	$(call print,Assembling:,$<,$@)
	$(V)$(AS) $(ASFLAGS_ARM9) -c $< -o $@
	
$(BUILD_DIR)/src/arm9/%.o: src/arm9/%.cpp
	$(call print,Compiling:,$<,$@)
	$(V)$(CC) $(CFLAGS_ARM9) -c $< -o $@

$(BUILD_DIR)/arm9.elf: $(ARM9_O_FILES) arm9.ld
	$(call print_single,Linking:,$@)
	$(V)$(LD) -T arm9.ld $(foreach dir,$(ARM9_SRC_DIRS) $(ARM9_ASM_DIRS),-L $(BUILD_DIR)/$(dir)) -o $@
