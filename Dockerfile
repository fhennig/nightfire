FROM rustembedded/cross:armv7-unknown-linux-gnueabihf

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes libasound2-dev:armhf && \
    apt-get install --assume-yes libudev-dev:armhf && \
    apt-get install --assume-yes libjack-jackd2-dev:armhf

# required to compile hidapi
ENV PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig/"