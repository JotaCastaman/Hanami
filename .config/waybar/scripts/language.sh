#!/bin/bash

while true; do
  LAYOUT=$(hyprctl devices -j | python3 -c "
import sys, json
d = json.load(sys.stdin)
kb = next((k for k in d['keyboards'] if 'hangsheng' in k['name']), None)
print(kb['active_keymap'] if kb else '')
" 2>/dev/null)

  if echo "$LAYOUT" | grep -qi "brazil\|portuguese"; then
    echo "BR"
  else
    echo "US"
  fi

  sleep 0.1
done
