#! /bin/bash

IMG=mediamanager

cd $(dirname $0)/..
docker build . -t $IMG
