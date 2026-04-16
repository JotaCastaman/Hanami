#!/bin/bash

SCROLL_WIDTH=25
SLEEP=0.4

offset=0
while true; do
  STATUS=$(playerctl --player=spotify status 2>/dev/null)

  if [ "$STATUS" = "Playing" ]; then
    ICON="󰎇 "
  elif [ "$STATUS" = "Paused" ]; then
    ICON="󰎇 "
  else
    echo '{"text": "󰎇  N/A", "class": "stopped"}'
    offset=0
    sleep 1
    continue
  fi

  ARTIST=$(playerctl --player=spotify metadata artist 2>/dev/null)
  TITLE=$(playerctl --player=spotify metadata title 2>/dev/null)
  TEXT="$ARTIST - $TITLE"
  LEN=${#TEXT}

  if [ $LEN -le $SCROLL_WIDTH ]; then
    echo "{\"text\": \"$ICON $TEXT\", \"class\": \"playing\"}"
    sleep 1
    continue
  fi

  PADDED="$TEXT    "
  DISPLAY="${PADDED:$offset:$SCROLL_WIDTH}"
  echo "{\"text\": \"$ICON $DISPLAY\", \"class\": \"playing\"}"

  offset=$(((offset + 1) % (LEN + 4)))
  sleep $SLEEP
done
