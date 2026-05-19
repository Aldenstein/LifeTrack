#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "==============================="
echo "   LifeTrack — Démarrage"
echo "==============================="

bash "$SCRIPT_DIR/startApi.sh"
echo ""
bash "$SCRIPT_DIR/startTunnel.sh"

echo ""
echo "==============================="
echo "   Screens actifs :"
screen -list | grep -E "api|tunnel" || echo "   (aucun)"
echo "==============================="
