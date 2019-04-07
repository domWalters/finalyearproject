#!/bin/bash

sudo apt install curl			# required in order to install rustup
sudo apt install build-essential	# required to get a C linker for cargo to work
curl https://sh.rustup.rs -sSf | sh	# rust installation

cd "$(dirname "$0")"
cd ../game

source $HOME/.cargo/env			# put cargo in the path variable
cargo run				# run the "compiler" to download rust crate dependencies

echo "If the line above this refers to a thread panic due to less than 1 argument, all dependencies successfully installed!"
echo "You can now run the program by the methods stated in README.md."
