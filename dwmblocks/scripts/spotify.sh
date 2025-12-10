#!/bin/sh

# Check if Spotify is running
if ! pgrep -x "spotify" > /dev/null; then
    echo "Spotify is not running"
    exit 0
fi

# If running, fetch metadata normally
TITLE=$(dbus-send --print-reply --dest=org.mpris.MediaPlayer2.spotify \
/org/mpris/MediaPlayer2 org.freedesktop.DBus.Properties.Get \
string:"org.mpris.MediaPlayer2.Player" string:"Metadata" \
| grep -A 1 "xesam:title" | tail -n1 | cut -d '"' -f 2)

ARTIST=$(dbus-send --print-reply --dest=org.mpris.MediaPlayer2.spotify \
/org/mpris/MediaPlayer2 org.freedesktop.DBus.Properties.Get \
string:"org.mpris.MediaPlayer2.Player" string:"Metadata" \
| grep -A 2 "xesam:artist" | tail -n1 | cut -d '"' -f 2)

echo "$ARTIST - $TITLE"

