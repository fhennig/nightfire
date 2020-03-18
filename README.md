# lumi

A server to control RGB lights connected to a Raspberry Pi.  It
includes a web interface and a couple of light animation modes.

It uses pi-blaster to set the lights.  A web UI is provided to set the
lights, under the hood it provides a GraphQL API.

Various modes are supported.


## Building and Installing

### System dependencies

some stuff.

sudo apt install libusb-1.0-0-dev:armhf
sudo apt install libudev-dev:armhf libfox-1.6-dev:armhf
  
### Builing for arm

    cargo build --release --target armv7-unknown-linux-gnueabihf --bin lumi
    
### Build the Website

    cd client
    npm start build
    
Copy the contents of the build directory to `/usr/local/lib/lumi/web/`.

    
### System Dependencies

- pi-blaster


### Installation

Put everything in /opt/lumi/


## Architecture

The server consists of various actors reading and writing a shared
state.  The lights are set in a thread which reads the color of the
lights from the state.  The state generates the color dynamically from
its internal state.

In other threads the state is modified to change the color of the
lights.  One thread runs a web server which serves a GraphQL API that
allows to modify the state.  A web frontend that interfaces with the
API is also provided.

There is a light struct that can only be owned by any single mode.
The mode can then internally have a thread that continuously updates
the light.

Every mode has an activate and deactivate function.  These can be used
to start and stop a thread.  Any mode can also implement other methods
that mutate internal state.

zu jeder mutation für einen mode gibt es dann auch ein query mit dem
man den zustand des modes abfragen kann.


### GraphQL API

The lights are controlled by modes.  At any time there is only one
mode active.

    mutation setLight {
      manualMode(settings: [
        {lightId: "light1", r: 1, g: 0, b: 1},
        {lightId: "light2", r:0}]) {
        ok
      }
    }

This mutation would activate the manualMode, and update its settings.

Different modes take different arguments, and arguments are frequently
optional:

    mutation pinkMode {
      pinkPulse
    }


## Ideas

- Implement a midi mode that listens in a thread for midi signals.
- a "random walk" for color hue
- a "beat" function for light intensity


## TODO

- Can I use
  [bluetooth-serial-port](https://github.com/Dushistov/bluetooth-serial-port)
  to connect to the controller via bluetooth?
- RTP-MIDI?
