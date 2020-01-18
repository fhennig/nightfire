# lumi

A server and web client to control RGB lights connected to a Raspberry
Pi.

It uses pi-blaster to set the lights.  A web UI is provided to set the
lights, under the hood it provides a GraphQL API.

Various modes are supported.

  
## Builing for arm

    cargo build --release --target armv7-unknown-linux-gnueabihf --bin lumi
    
## Build the Website

    cd client
    npm start build
    
Copy the contents of the build directory to `/usr/local/lib/lumi/web/`.

    
## System Dependencies

- pi-blaster
- nginx


## Installation

- Put binary in `/usr/local/bin/lumi`
- Put web files in `/usr/local/lib/lumi/web/`
- Put the systemd unit file in `/etc/systemd/system/lumi.service`
- Put the nginx file (`lumi`) in the correct place
- Put the config file in `/etc/lumi/conf.yaml`


## GraphQL API

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


## Internal workings

There is a light struct that can only be owned by any single mode.
The mode can then internally have a thread that continuously updates
the light.

Every mode has an activate and deactivate function.  These can be used
to start and stop a thread.  Any mode can also implement other methods
that mutate internal state.

zu jeder mutation für einen mode gibt es dann auch ein query mit dem
man den zustand des modes abfragen kann.


## Ideas

- Implement a midi mode that listens in a thread for midi signals.
- a "random walk" for color hue
- a "beat" function for light intensity
- a pulsating/heart beat function for light intensity
- a rainbow function for color hue


## TODO

- move the palette color representation
- implement query structure to query mode settings
