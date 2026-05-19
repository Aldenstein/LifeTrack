#!/bin/bash

SCREEN_NAME="tunnel"

if screen -list | grep -q "\.${SCREEN_NAME}"; then
    screen -S "$SCREEN_NAME" -X quit
    echo "[✓] Tunnel arrêté"
else
    echo "[!] Aucun screen '$SCREEN_NAME' actif"
fi
