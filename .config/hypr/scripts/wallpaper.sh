#!/bin/bash

WALLPAPER_DIR="$HOME/Pictures/Wallpaper"
WALLPAPERS=($(ls "$WALLPAPER_DIR"))
CURRENT=$(swww query | grep -oP '(?<=image: ).*')

for i in "${!WALLPAPERS[@]}"; do
    if [[ "$WALLPAPER_DIR/${WALLPAPERS[$i]}" == "$CURRENT" ]]; then
        NEXT=${WALLPAPERS[$(( (i + 1) % ${#WALLPAPERS[@]} ))]}
        break
    fi
done

NEXT=${NEXT:-${WALLPAPERS[0]}}

swww img "$WALLPAPER_DIR/$NEXT" \
    --transition-type grow \
    --transition-pos center \
    --transition-duration 1.5 \
    --transition-fps 60
