# lumi

`lumi` allows you to control RGB lights with a playstation 3
controller (dualshock six axis).


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

The binary and config file needs to be in `/opt/lumi`.

#### systemd

put the following in `/etc/systemd/system/lumi.service`:

    [Unit]
    Description=The Lumi light server
    After=pi-blaster.service
    Requires=pi-blaster.service
    
    [Service]
    Type=simple
    Environment=RUST_BACKTRACE=1
    WorkingDirectory=/opt/lumi
    ExecStart=/opt/lumi/lumi
    
    [Install]
    WantedBy=multi-user.target



## Ideas

- Implement a midi mode that listens in a thread for midi signals.
- a "random walk" for color hue
- a "beat" function for light intensity
- Can I use
  [bluetooth-serial-port](https://github.com/Dushistov/bluetooth-serial-port)
  to connect to the controller via bluetooth?
- RTP-MIDI?
- More different EnvMasks

### Controller Mode

- Make L3 press switch between black vs white when stick not moved.
- pos mask only active when stick moved away from center.
-> Then we don't need a Black inactive mode

### New Mode

A new mode where both sticks are a "blob" that is moved around.  One
is red, the other is blue.  Can be a nice mix of colors.  Make the
controller very symmetrical, have one side for each blob.

A new mode that allows selecting colors for the individual lights.
Hue can again be selected with the right stick.  saturation and value
with the triggers.  The left stick indicates which stick color should
be changed.

### Mode Selection

Allow mode selection when holding the PS button and moving any of the
sticks.  Indicate current mode by color (6 modes -> six colors)

### OSC Interface

I could implement a OSC interface to allow to control the lights over
the network.  This way it would make it easy to control the lights
from any device that supports OSC, and there are a few libraries in
different languages:
- [rosc](https://docs.rs/rosc/0.3.0/rosc/) can be used on the pi to
  provide the interface.  I can also use it to write a driver for the
  controller that runs on the pi and connects to the controller if it
  is plugged in.  I could run the same driver on a desktop PC and send
  the controller signal over network.  It needs to be paired with UDP
  for transmission.
- [JavaOSC](https://github.com/hoijui/JavaOSC) could easily be used to
  write a little java program that reads midi on windows and writes it
  to the network.

I should seperate into a seperate sub project (directory) a driver for
the controller.

I should have the functionality of the lights exposed as an OSC
interface, and I should have the controller driver take care of the
logic to turn button presses etc into OSC messages to the lights.

This way the driver can have seperate settings, which things it should
modify (i.e. which messages to send) and I can also send messages from
another software.  I.e. I just modify the color with the stick from
the controller, but another software sets the light intensity in the
rythm of the music.
