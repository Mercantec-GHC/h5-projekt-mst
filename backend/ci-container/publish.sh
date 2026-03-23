#!/bin/bash

tag=sfja/h5-mst-ci-backend

set -xe

docker build -t $tag .

docker push $tag

