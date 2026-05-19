#!/bin/bash

SCREEN_NAME="api"

if screen -list | grep -q "\.${SCREEN_NAME}"; then
    screen -S "$SCREEN_NAME" -X quit
    echo "[✓] API arrêtée"
else
    echo "[!] Aucun screen '$SCREEN_NAME' actif"
fi
