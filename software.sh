#!/bin/bash

#gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2 download this cross compiler toolchain for gdb

cargo update

rustup install nightly
rustup target add thumbv7em-none-eabi --toolchain nightly

sudo apt install gcc-arm-none-eabi gdb-multiarch

#git clone https://github.com/utzig/lm4tools.git
#cd lm4tools/lm4flash
#make
#sudo cp lm4flash /usr/local/bin


#Debug:
#run debug_launch command in Makefile in 1 terminal and gdb in another terminal
#commands:
#target extended-remote : 3333 or target remote localhost:3333
#monitor reset halt or monitor reset init
#load
#continue
#Follow openocd.gdb commands as well