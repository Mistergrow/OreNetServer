# OreNetServer

**OreNetServer** ist ein flexibler Ore Mining-Pool-Server auf der Solana-Blockchain. OreNetServer ist sowohl im LAN als auch global nutzbar.

## Features

- **LAN- und globaler Einsatz**: OreNetServer kann lokal (LAN) oder global über das Internet betrieben werden, um Mining-Clients zu verwalten.
- **Dynamische Transaktionsgebühren**: Berechnung der Gebühren in Echtzeit basierend auf Daten von QuickNode, Helius und Alchemy, um die Effizienz des Mining-Prozesses zu maximieren.
- **Multi-Threading für Audio-Benachrichtigungen**: Audio-Benachrichtigungen werden in separaten Threads ausgeführt, um die Performance des Servers zu erhalten.
- **Webhooks für Benachrichtigungen**: Echtzeit-Benachrichtigungen über Slack und Discord für Mining-Fortschritte und Ergebnisse.

## Installation

### 1. Rust-Umgebung einrichten

Stelle sicher, dass Rust und die Solana-CLI installiert sind:

- [Rust Installation](https://www.rust-lang.org/tools/install)
- [Solana CLI Installation](https://docs.solana.com/cli/install-solana-cli-tools)

Klone das Repository:

```bash
git clone https://github.com/username/OreNetServer.git
cd OreNetServer
