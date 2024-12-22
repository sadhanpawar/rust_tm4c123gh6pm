
all:
	@echo "use other commands present in makefile"

release:
	cargo +nightly build -Zbuild-std=core,alloc --release

debug:
	cargo +nightly build -Zbuild-std=core,alloc 

debug_launch:
	openocd -f board/ek-tm4c123gxl.cfg

gdb:
	arm-none-eabi-gdb target/thumbv7em-none-eabi/debug/ti_tm4c123

flash:
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabi/release/ti_tm4c123 output.bin
	mv output.bin target/thumbv7em-none-eabi/release/
	lm4flash target/thumbv7em-none-eabi/release/output.bin
	
clean:
	cargo clean
	rm -rf target/thumbv7em-none-eabi/release/output.bin