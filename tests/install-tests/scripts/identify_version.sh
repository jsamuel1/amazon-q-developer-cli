#!/bin/sh
# Script to identify the distribution and version

if [ -f /etc/os-release ]; then
    . /etc/os-release
    echo "$NAME $VERSION_ID"
elif [ -f /etc/lsb-release ]; then
    . /etc/lsb-release
    echo "$DISTRIB_ID $DISTRIB_RELEASE"
elif [ -f /etc/debian_version ]; then
    echo "Debian $(cat /etc/debian_version)"
elif [ -f /etc/redhat-release ]; then
    cat /etc/redhat-release
elif [ -f /etc/alpine-release ]; then
    echo "Alpine $(cat /etc/alpine-release)"
else
    echo "Unknown distribution"
fi
