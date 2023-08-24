#! /bin/sh

NAME=mediamanager
IMG=mediamanager:latest

docker stop $NAME
docker rm $NAME

docker run -d \
    -p "8080:8080" \
    -v "${HOME}/Videos/mediamanager:/var/lib/mediamanager" \
    --device="/dev/sr0:/dev/sr0" \
    --privileged \
    --restart always \
    --name $NAME \
    $IMG

