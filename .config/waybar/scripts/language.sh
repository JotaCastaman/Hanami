#!/bin/bash

while true; do
  LAYOUT=$(swaymsg -t get_inputs | python3 -c "
import sys, json
d = json.load(sys.stdin)
kb = next((i for i in d if i.get('type') == 'keyboard' and 'hangsheng' in i.get('identifier', '').lower()), None)
print(kb['xkb_active_layout_name'] if kb else '')
" 2>/dev/null)

  if echo "$LAYOUT" | grep -qi "brazil\|portuguese"; then
    echo "BR"
  else
    echo "US"
  fi

  sleep 0.1
done
