#!/bin/sh

set -ex

cd "$(dirname "$0")"

export LIBGL_ALWAYS_SOFTWARE=true

run=env
if [ -z $DBUS_SESSION_BUS_ADDRESS ] && which dbus-launch; then
  run=dbus-launch  
fi

$run Xorg -noreset +extension GLX +extension RANDR +extension RENDER -logfile /dev/stdout -config ./xorg.conf :5
