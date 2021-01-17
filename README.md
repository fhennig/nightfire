# nightfire

`nightfire` is an audio to light project. The goal is to create a tool
that allows for expressive visualization of sound, combining user
input and automatic audio processing. This is the repository
containing the code, an overview of features and demos can be found at
[nightfire.io](https://nightfire.io).

## Structure

This project is developed in rust, and contains 5 crates (organized in
[workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)).

**nighfire**: The `nightfire` crate contains the core code for audio
processing and light modeling.

**nf_lichtspiel**: Uses the `nightfire` crate and contains code to
handle `pi-blaster` (interfacing with the LEDs) and HID handling (PS
controller).  It also contains the code to interact with `jackd`, to
read out microphone inputs.

rest: **mixxx_data** contains a tool to extract features from songs from
the Mixxx DJ software. *beat_trainer* contains some python code to
inspect said data and some experimental neural networks trained on
them. **bpm_tapper** contains a small tool to tap BPM along to a song
and visualize them.

*confs*: contains some sample configuration files to use for running
the `nightfire` binary.

## Building and Installing


The main binary is built with:

    cargo build --bin nightfire
    
Which compiles the full LED/Sound/Controller handling component.
  
### Builing for arm

Cross-compilation for arm uses [cross](https://github.com/rust-embedded/cross),
which builds `armhf` binaries in docker container.  This also requires that 
docker is installed, and the build image is build with 

    docker build -t nightfire-build-armhf:latest .

Then, the binary for nightfire can be built as such:

    cross build --release --target armv7-unknown-linux-gnueabihf --bin nightfire

The file is output in `target/armv7-unknown-linux-gnueabihf/release/nightfire`.
    
### Raspberry Pi Setup

These things need to be installed on the Raspberry Pi.

- `pi-blaster`: Is needed to set the pins, controlling the LEDs.
  pi-blaster can be found on
  [github](https://github.com/sarfata/pi-blaster)
- `libjack0`: This dependency is required because nightfire is compiled
  with jack support.  You don't actually need to install jack if you
  don't want to do audio processing, but the library still needs to be
  there.  Can be installed with `sudo apt install libjack0`.

The binary and config file (`conf.yaml`) needs to be in `/opt/nightfire`.

#### systemd

put the following in `/etc/systemd/system/nightfire.service`:

    [Unit]
    Description=The Nightfire light server
    After=pi-blaster.service
    Requires=pi-blaster.service
    
    [Service]
    Type=simple
    Environment=RUST_BACKTRACE=1
    WorkingDirectory=/opt/nightfire
    ExecStart=/opt/nightfire/nightfire
    
    [Install]
    WantedBy=multi-user.target


## Configuration

The software is configurated in a `conf.yaml` file that should be in
the working directory.  The config file could look like this:

    audio-in: system:capture_1
    pi-blaster: /dev/pi-blaster
    lights:
      Top:
        r: 14
        g: 15
        b: 18
      Bottom:
        r: 23
        g: 25
        b: 24
      Left:
        r: 17
        g: 27
        b: 22
      Right:
        r: 10
        g: 9
        b: 11

`audio-in` is a `device:port` entry that specifies which jack port to
read from.  It can also be set to `off` to disable reading audio input.
Do this if jack is not setup on the device.

`pi-blaster` refers to the path of the pi-blaster device path.

The numbers in the lights refer to GPIO pin numbers.  The number of a
pin can be found [here](https://pinout.xyz/).
