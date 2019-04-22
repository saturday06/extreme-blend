#!/bin/sh

set -ex

run=env
if [ -z $DBUS_SESSION_BUS_ADDRESS ]; then
  run=dbus-launch  
fi

$run Xorg -noreset +extension GLX +extension RANDR +extension RENDER -logfile /dev/stdout -config ./xorg.conf :0
