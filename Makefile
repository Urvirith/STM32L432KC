# Make Script For Embedded Rust
AS			:= arm-none-eabi-as			# assembler
LD 			:= arm-none-eabi-ld 		# linker
OBJ 		:= arm-none-eabi-objcopy	# Object Copy

# -Os				Optimize for Size
# -mcpu=cortex-m4	Compile for the ARM M4 Processor
# mthumb			Target the MTHUMB Instruction Set
ASFLAGS		:= -mcpu=cortex-m4 -mthumb
LDFLAGS 	:= -T 
OBJFLAGS	:= -O binary

#	EXAMPLE OF AUTOMATIC VARIABLES
#	%.o: %.c %.h common.h
#		$(CC) $(CFLAGS) -c $<
#
#	$@ Looks at the target
#	(Target)
#	%.o: 			%.c %.h common.h
#	
#	$< Looks at the first source
#			(First Source)
#	%.o: 	%.c 					%.h common.h
#		$(CC) $(CFLAGS) -c $<
#	$^ Looks at the first source
#			(All Source)
#	%.o: 	%.c %.h common.h
#		$(CC) $(CFLAGS) -c $^

STARTUP_DIR := src/startup/
LINKER_DIR  := src/linker/
BLD_DIR	  	:= build/

release: build/main.bin

# Copy to a bin file
build/main.bin: build/main.elf
	$(OBJ) $(OBJFLAGS) $^ $@

# Build An ELF 
$(BLD_DIR)main.elf: $(LINKER_DIR)gcc_arm.ld $(BLD_DIR)main.o $(BLD_DIR)startup.o
	$(LD) -Os -s $(LDFLAGS) $^ -o $@

# Build Dependances
$(BLD_DIR)startup.o: $(STARTUP_DIR)startup_ARMCM4.S
	$(AS) $< $(ASFLAGS) -o $@

# Build The Rust Project, .cargo and Cargo.Toml hold the flags for this
$(BLD_DIR)main.o:
	cargo build --release

# Clean The Build Folder To Allow For A Complete Rebuild
clean:
	rm -f $(BLD_DIR)*.o
	rm -f $(BLD_DIR)*.elf
	rm -f $(BLD_DIR)*.bin
	cargo clean

# Flash the board us st-flash utility
flash:
	st-flash write $(BLD_DIR)main.bin 0x08000000