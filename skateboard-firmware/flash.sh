#!/bin/bash

set -xe

# run this on the host: esp_rfc2217_server -v -p 4000 /dev/ttyACM0

docker run --rm -v $PWD:/project -w /project -u $UID -e HOME=/tmp espressif/idf idf.py -p 4000:4000 flash

