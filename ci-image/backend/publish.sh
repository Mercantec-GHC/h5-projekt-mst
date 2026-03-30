#!/bin/bash

tag=sfja/h5-mst-ci:backend-ci

set -xe

docker build -t $tag .

docker push $tag

