#!/usr/bin/env bash

# Start ore mining private pool server

set -e

SRV="$HOME/miner/ore-private-pool-srv/target/release/ore-ppl-srv"
MKP="$HOME/.config/solana/id.json"

# .env import
# Pfad zur .env-Datei
envFilePath="$HOME/miner/ore-private-pool-srv/.env"

# Überprüfen, ob die .env-Datei existiert
if [[ -f "$envFilePath" ]]; then
    # Die .env-Datei laden
    export $(grep -v '^#' "$envFilePath" | xargs)

    echo "Die .env-Datei wurde erfolgreich geladen und die Umgebungsvariablen wurden gesetzt."
else
    echo "Die .env-Datei wurde nicht gefunden: $envFilePath"
    exit 1
fi

# Dynamische Gebühren-URL setzen, falls aktiviert
# Standardmäßige dynamische Gebühren-URL. Nächste Zeile auskommentieren, um dynamische Gebühren zu aktivieren
# DYNAMIC_FEE_URL="https://api.devnet.solana.com/"

BUFFER_TIME=5
RISK_TIME=0
PRIORITY_FEE=100
PRIORITY_FEE_CAP=10000

EXP_MIN_DIFF=8
SLACK_DIFF=25
XTR_FEE_DIFF=29
XTR_FEE_PCT=100

# Wähle den Server-Modus
echo "Ore Private Server"
echo "1) Priority-fee mode"
echo "2) Dynamic-fee mode"
read -p "Modus auswählen: " mode

case $mode in
    1)
        # Priority-fee mode
        CMD="$SRV --buffer-time $BUFFER_TIME \
                   --risk-time $RISK_TIME \
                   --priority-fee $PRIORITY_FEE \
                   --priority-fee-cap $PRIORITY_FEE_CAP \
                   --expected-min-difficulty $EXP_MIN_DIFF \
                   --slack-difficulty $SLACK_DIFF \
                   --extra-fee-difficulty $XTR_FEE_DIFF \
                   --extra-fee-percent $XTR_FEE_PCT"
        ;;
    2)
        # Dynamic-fee mode
        if [[ -n "$DYNAMIC_FEE_URL" && "$DYNAMIC_FEE_URL" != "YOUR_RPC_URL_HERE" ]]; then
            CMD="$SRV --buffer-time $BUFFER_TIME \
                       --dynamic-fee \
                       --dynamic-fee-url $DYNAMIC_FEE_URL \
                       --priority-fee-cap $PRIORITY_FEE_CAP \
                       --expected-min-difficulty $EXP_MIN_DIFF \
                       --slack-difficulty $SLACK_DIFF \
                       --extra-fee-difficulty $XTR_FEE_DIFF \
                       --extra-fee-percent $XTR_FEE_PCT"
        else
            echo "Dynamische Gebühren-URL ist nicht gesetzt oder enthält einen Platzhalterwert."
            exit 1
        fi
        ;;
    *)
        echo "Ungültige Auswahl. Bitte 1 oder 2 eingeben."
        exit 1
        ;;
esac

# Zeige den Befehl an
echo $CMD

# Schleife zum Wiederholen des Befehls bei Fehlern
until bash -c "$CMD"; do
    echo "Fehler beim Starten des Server-Befehls. Neustart..."
    sleep 2
done
