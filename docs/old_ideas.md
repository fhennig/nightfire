# Old Ideas

Here I want to put some old ideas that might come in handy at some
point again, but that I don't plan to work on for now.

## OSC Interface

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

----

I could make a seperate module for the controlling of the lights, that
just receives color over OSC.  for each light I could have an OSC
address, and specify for each address the pins for RGB.  This could be
fully specified in a yaml file that the module reads on startup.

In a seperate yaml file on the server I map coordinates to adresses.
