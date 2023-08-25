#! /bin/bash

VERSION=1.17.4
BIN_PKG=makemkv-bin-$VERSION
OSS_PKG=makemkv-oss-$VERSION

curl https://www.makemkv.com/download/$BIN_PKG.tar.gz --output $BIN_PKG.tar.gz
curl https://www.makemkv.com/download/$OSS_PKG.tar.gz --output $OSS_PKG.tar.gz

apt-get install -y build-essential pkg-config libc6-dev libssl-dev libexpat1-dev libavcodec-dev libgl1-mesa-dev qtbase5-dev zlib1g-dev

tar xvf $OSS_PKG.tar.gz
cd $OSS_PKG
./configure --disable-gui
make install
cd ..

tar xvf $BIN_PKG.tar.gz
cd $BIN_PKG
mkdir tmp
touch tmp/eula_accepted
make install
