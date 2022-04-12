#! /bin/bash
set -ex

cd toolchain
docker build --network=host -t ewpratten/kobo-cross-armhf -f kobo-cross-armhf.dockerfile .
cd ..