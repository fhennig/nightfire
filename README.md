# lumi

So far I implemented reading a YAML config and setting lights with
pi-blaster.

Next steps:
- Setup instructions/Makefile to compile for RPi.
- Add config option for the pi-blaster file and the config file
- Add the graphQL interface (design it first?)
  - Setup HTTP-server
  
## Builing for arm

    cargo build --release --target armv7-unknown-linux-gnueabihf --bin lumi
