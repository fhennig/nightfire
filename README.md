# lumi

A server and web client to control RGB lights connected to a Raspberry
Pi.

  
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


## Idea

Neue Idee für die API:  Es gibt für jeden Mode einen Endpoint.  Der
zum lichter setzen würde dann so umgebaut:


mutation setLight {
  manualMode(settings: [
    {lightId: "light1", r: 1, g: 0, b: 1},
    {lightId: "light2", r:0}]) {
    ok
  }
}

Mit input-objekten die unterspezifiziert sein können.  Um den anderen
mode zu setzen würde man dann aufrufen:

mutation pinkMode {
  pinkPulse() {
    ok
  }
}

Das Ding ist, der manual mode braucht keine loop, der andere schon.

Jeder mode braucht eine activate und deactivate funktion.  in der
activate funktion kann der thread gespawnt werden und in der
deactivate funktion wird er dann gestoppt und gejoint.  Timer oder
ähnliches können gestoppt werden.

Falls ein mode mit anderen settings gesetzt wird, und er schon
aktiviert ist, werden die settings einfach on the fly geändert.

zu jeder mutation für einen mode gibt es dann auch ein query mit dem
man den zustand des modes abfragen kann.

Jeder mode hat dabei aber seine eigenen farben etc und es wird
nichts geteilt.  zB braucht der pink mode für jedes licht eine farbe,
dann rechnet er da noch den envelope drauf und setzt die farbe.  Die
farbe wird aber intern gespeichert.  Die gedimmte farbe wird dann in
das light_model geschrieben.

Ein MidiMode könnte dann in seinem thread auf irgendwelche midi
signale lauschen.

Things I want:
- a "random walk" for color hue
- a "beat" function for light intensity
- a pulsating/heart beat function for light intensity
- a rainbow function for color hue


## TODO

- Quality of Life improvements
  - debug mode with UI instead of pi-blaster
  - load config from different paths if one path is not set.
  - maybe a local serving mode that serves the web stuff too (without
    nginx).
- Features
  - Implement the modes in the states and the activate/deactivate
    stuff
  - Adapt the GraphQL API accordingly.
