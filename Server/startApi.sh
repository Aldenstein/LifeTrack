#!/bin/bash

API_DIR="$HOME/LifeTrack/BDD"
API_BIN="$API_DIR/target/release/lifetrackdb"
SCREEN_NAME="api"

# Vérifier si le screen existe déjà
if screen -list | grep -q "\.${SCREEN_NAME}"; then
    echo "[!] L'API tourne déjà (screen '$SCREEN_NAME' actif)"
    echo "    Pour y accéder : screen -r $SCREEN_NAME"
    exit 1
fi

# Vérifier que le binaire existe
if [ ! -f "$API_BIN" ]; then
    echo "[!] Binaire introuvable : $API_BIN"
    echo "    Lance d'abord : cd $API_DIR && cargo build --release"
    exit 1
fi

# Vérifier que le .env existe
if [ ! -f "$API_DIR/.env" ]; then
    echo "[!] Fichier .env manquant dans $API_DIR"
    exit 1
fi

echo "[*] Démarrage de l'API LifeTrack..."
screen -dmS "$SCREEN_NAME" bash -c "cd $API_DIR && export \$(cat .env | xargs) && $API_BIN"
sleep 1

if screen -list | grep -q "\.${SCREEN_NAME}"; then
    echo "[✓] API démarrée (screen '$SCREEN_NAME')"
    echo "    Logs : screen -r $SCREEN_NAME  |  Quitter : Ctrl+A puis D"
else
    echo "[✗] Échec du démarrage — vérifie les logs avec : screen -r $SCREEN_NAME"
    exit 1
fi
