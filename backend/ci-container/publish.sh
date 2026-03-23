#!/bin/bash

version=$1

if [[ $version == "" ]]; then
    echo "Please specify version"
    echo "./publish.sh <version e.g. 1.0>"
    exit 1
fi

tag=sfja/h5-mst-ci-backend:$version

set -xe

docker build -t $tag .

docker push $tag

