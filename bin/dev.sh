#!/bin/sh
# Start the Yew dev server and show all errors
cd "$(dirname "$0")/../yew"
exec trunk serve 