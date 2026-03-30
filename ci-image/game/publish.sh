#!/bin/bash

tag=sfja/h5-mst-ci:game-ci

set -xe

docker build -t $tag .

docker push $tag

