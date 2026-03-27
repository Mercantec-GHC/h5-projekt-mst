#!/bin/bash

user=$1

if [[ $user == "" ]]; then
    echo "Please specify server user"
    echo "./deploy.sh <user>"
    exit 1
fi

compose_file=docker-compose.yml

host=10.133.51.127

set -xe

scp $compose_file $user@$host:/home/$user/

# echo sudo -S docker compose down
# echo sudo docker compose up -d
# ssh $user@$host

