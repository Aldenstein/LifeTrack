#!/bin/bash

SCREEN_NAME="tunnel"
TUNNEL_NAME="lifetrack"   # <-- remplace par le nom de ton tunnel Cloudflare

# Vérifier si cloudflared est installé
if ! command -v cloudflared &> /dev/null; then
    echo "[!] cloudflared n'est pas installé"
    echo ""
    echo "    Installation :"
    echo "    curl -L https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64 -o cloudflared"
    echo "    chmod +x cloudflared && sudo mv cloudflared /usr/local/bin/"
    echo "    cloudflared tunnel login"
    echo "    cloudflared tunnel create $TUNNEL_NAME"
    echo "    cloudflared tunnel route dns $TUNNEL_NAME ton.domaine.com"
    echo ""
    exit 1
fi

# Vérifier si le screen existe déjà
if screen -list | grep -q "\.${SCREEN_NAME}"; then
    echo "[!] Le tunnel tourne déjà (screen '$SCREEN_NAME' actif)"
    echo "    Pour y accéder : screen -r $SCREEN_NAME"
    exit 1
fi

echo "[*] Démarrage du tunnel Cloudflare..."
screen -dmS "$SCREEN_NAME" bash -c "cloudflared tunnel run $TUNNEL_NAME"
sleep 2

if screen -list | grep -q "\.${SCREEN_NAME}"; then
    echo "[✓] Tunnel démarré (screen '$SCREEN_NAME')"
    echo "    Logs : screen -r $SCREEN_NAME  |  Quitter : Ctrl+A puis D"
else
    echo "[✗] Échec du démarrage — vérifie les logs avec : screen -r $SCREEN_NAME"
    exit 1
fi
