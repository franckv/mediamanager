# Media ripping tool

## Description

This script auto-detect media inserted in optical drive /dev/sr[0-9] and run the appropriate ripping command in the background

## Usage

Use the udev & systemd provided scripts for full automation.

To run the script manually, use the scripts/mediamanager command:
~~~
Usage: mediamanager [DVD|CD|BR] sr[0-9]
~~~

## Installation

- Run scripts/build.sh to build docker container
- Run scripts/run.sh to start container
- For udev automation do the following on the docker host:
  - copy scripts/udev & scripts/systemd to /usr/local/lib/
  - copy scripts/mediamanager to /usr/local/bin/

## Configuration

- Update default configuration in *config/default.config* **before** building the container
- Update default user in */scripts/systemd/system/mediamanager@.service*

## Troubleshooting

### Error "The program can't find any usable optical drives. Failed to open disc"

Make sure "sg" module is loaded
~~~
modprobe sg
~~~
