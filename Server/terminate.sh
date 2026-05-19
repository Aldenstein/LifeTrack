#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "==============================="
echo "   LifeTrack — Arrêt"
echo "==============================="

bash "$SCRIPT_DIR/stopTunnel.sh"
bash "$SCRIPT_DIR/stopApi.sh"

echo ""
echo "Screens restants :"
screen -list | grep -E "api|tunnel" || echo "   (aucun) — tout est arrêté ✓"
echo "==============================="
