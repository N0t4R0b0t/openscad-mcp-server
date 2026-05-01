#!/bin/bash
uid=$(id -u)
export XDG_RUNTIME_DIR="/run/user/$uid"
export DBUS_SESSION_BUS_ADDRESS="unix:path=/run/user/$uid/bus"
exec "$(dirname "$0")/target/release/openscad-mcp-server"
