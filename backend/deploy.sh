#!/bin/bash

user=$1

if [[ $user == "" ]]; then
    echo "Please specify server user"
    echo "./deploy.sh <user>"
    exit 1
fi

host=10.133.51.127

set -xe

tar czvf mst-backend.tar.gz deploy/*

scp mst-backend.tar.gz $user@$host:/home/$user/

ssh $user@$host -C 'tar xvf mst-backend.tar.gz && cd deploy/ && ./install.sh'

