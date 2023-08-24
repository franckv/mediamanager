#! /bin/sh

NAME=mediamanager
IMG=mediamanager:latest

docker stop $NAME
docker rm $NAME

docker run -d \
    -p "8080:8080" \
    -v "${HOME}/Videos/mediamanager:/var/lib/mediamanager" \
    --device="/dev/sr0:/dev/sr0" \
    --device="/dev/sr1:/dev/sr1" \
    --device="/dev/sr2:/dev/sr2" \
    --device="/dev/sr3:/dev/sr3" \
    --device="/dev/sr4:/dev/sr4" \
    --device="/dev/sr5:/dev/sr5" \
    --device="/dev/sr6:/dev/sr6" \
    --device="/dev/sr7:/dev/sr7" \
    --device="/dev/sr8:/dev/sr8" \
    --device="/dev/sr9:/dev/sr9" \
    --privileged \
    --restart always \
    --name $NAME \
    $IMG

