# .env import
# Pfad zur .env-Datei
$envFilePath = "C:\path\to\your\.env"

# Überprüfe, ob die Datei existiert
if (Test-Path $envFilePath) {
    # Datei Zeile für Zeile lesen
    Get-Content $envFilePath | ForEach-Object {
        # Kommentarzeilen (beginnend mit '#') und leere Zeilen ignorieren
        if ($_ -match '^\s*$' -or $_ -match '^\s*#') {
            return
        }
        # Jede Zeile in Schlüssel und Wert aufteilen
        $parts = $_ -split '=', 2
        $key = $parts[0].Trim()
        $value = $parts[1].Trim().Trim('"')  # Anführungszeichen entfernen
        
        # Umgebungsvariable setzen
        [System.Environment]::SetEnvironmentVariable($key, $value)
    }

    Write-Host -ForegroundColor Green "`n`n`t`t`tDie .env-Datei wurde erfolgreich geladen und die Umgebungsvariablen wurden gesetzt."`n`n`n`n
} else {
    Write-Host -ForegroundColor Red "Die .env-Datei wurde nicht gefunden: $envFilePath"`n`n`n`n
    $host.Exit()
}

# Teste eine Umgebungsvariable
Write-Host "WALLET:`t`t`t$([System.Environment]::GetEnvironmentVariable('WALLET_PATH'))"`n
Write-Host "RPC:`t`t`t$([System.Environment]::GetEnvironmentVariable('RPC_URL'))"`n
Write-Host "RPC WSS:`t`t$([System.Environment]::GetEnvironmentVariable('RPC_WS_URL'))"`n
Write-Host "HELIUS DYN_FEE:`t`t$([System.Environment]::GetEnvironmentVariable('HELIUS_URL'))"`n
Write-Host "Quicknode DYN_FEE:`t$([System.Environment]::GetEnvironmentVariable('QUICKNODE_URL'))"`n
Write-Host "Alchemy DYN_FEE:`t$([System.Environment]::GetEnvironmentVariable('ALCHEMY_URL'))"`n
Write-Host "Triton DYN_FEE:`t`t$([System.Environment]::GetEnvironmentVariable('TRITON_URL'))"`n`n`n
Write-Host "SLACK:`t`t`t$([System.Environment]::GetEnvironmentVariable('SLACK_WEBHOOK'))"`n
Write-Host "DISCORD:`t`t$([System.Environment]::GetEnvironmentVariable('DISCORD_WEBHOOK'))"`n`n`n

$null = Read-Host 'Zum Fortfahren ENTER drücken'
cls

$SRV = "C:\path\to\your\ore-ppl-srv.exe"
$MKP = "C:\path\to\your\Ore3.json"

# Standardmäßige dynamische Gebühren-URL. Nächste Zeile auskommentieren, wenn du den dynamischen Modus aktivieren möchtest
$DYNAMIC_FEE_URL = "https://example-rpc.solana.com/"

$BUFFER_TIME = 6
$RISK_TIME = 2
$PRIORITY_FEE = 18000
$PRIORITY_FEE_CAP = 39000
$EXP_MIN_DIFF = 15
$SLACK_DIFF = 23
$XTR_FEE_DIFF = 29
$XTR_FEE_PCT = 50

# Wähle den Server-Modus
$mode = Read-Host "Ore Private Server`nPriority-fee mode 1`nDynamic-fee mode 2`nModus auswählen:"

switch ($mode) {
    1 {
        # Priority-fee mode
        $CMD = "$SRV --buffer-time $BUFFER_TIME --risk-time $RISK_TIME --priority-fee $PRIORITY_FEE --priority-fee-cap $PRIORITY_FEE_CAP --expected-min-difficulty $EXP_MIN_DIFF --slack-difficulty $SLACK_DIFF --extra-fee-difficulty $XTR_FEE_DIFF --extra-fee-percent $XTR_FEE_PCT"
    }
    2 {
        # Dynamic-fee mode
        if ($DYNAMIC_FEE_URL -ne "YOUR_RPC_URL_HERE" -and -not [string]::IsNullOrEmpty($DYNAMIC_FEE_URL)) {
            $CMD = "$SRV --buffer-time $BUFFER_TIME --dynamic-fee --dynamic-fee-url $DYNAMIC_FEE_URL --priority-fee-cap $PRIORITY_FEE_CAP --expected-min-difficulty $EXP_MIN_DIFF --slack-difficulty $SLACK_DIFF --extra-fee-difficulty $XTR_FEE_DIFF --extra-fee-percent $XTR_FEE_PCT"
        } else {
            Write-Host "Dynamische Gebühren-URL ist nicht gesetzt oder verwendet einen Platzhalter."
            exit
        }
    }
    default {
        Write-Host "Ungültige Auswahl. Bitte 1 oder 2 eingeben."
        exit
    }
}

# Zeige den Befehl an
Write-Host $CMD

# Schleife zum Wiederholen des Befehls bei Fehlern
while ($true) {
    try {
        Invoke-Expression $CMD
        break # Schleife verlassen, wenn erfolgreich
    } catch {
        Write-Host "Fehler beim Starten des Server-Befehls. Neustart..."
        Start-Sleep -Seconds 2
    }
}
