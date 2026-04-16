#!/bin/sh

IFACE="enp34s0"

if cat /sys/class/net/$IFACE/operstate | grep -q "up"; then
    echo " 󰈁"
else
    echo " 󰈂" 
fi
