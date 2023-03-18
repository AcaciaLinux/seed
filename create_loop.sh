#!/bin/sh

if [ "$#" -ne 2 ]; then
	echo "usage: $0 <loopback file> <size in GB>"
	exit -1
fi

if [ -f $1 ]; then
	echo "Loopback file $1 does already exist, skipping creation..."
else
	dd if=/dev/zero of=$1 bs=1G count=$2
fi

if [[ $(losetup -a) == *"$1"* ]]; then
	echo "File is already mounted!"
	exit 0
fi

losetup -f $1

echo "Done:"
losetup -a

