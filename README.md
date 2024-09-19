# OreNetServer

**OreNetServer** ist ein zentraler Server für das Mining von Ore (ORE) auf der Solana-Blockchain. Der Server ist flexibel einsetzbar und kann sowohl im LAN als auch in globalen Netzwerken verwendet werden. OreNetServer verwaltet Mining-Clients, synchronisiert deren Aktivitäten und berechnet dynamische Transaktionsgebühren über verschiedene RPC-Dienste.

## Features

- **LAN- und globaler Einsatz**: OreNetServer funktioniert sowohl in einem lokalen Netzwerk (LAN) als auch über das Internet, um Mining-Clients zu verwalten.
- **Dynamische Transaktionsgebühren**: Unterstützt QuickNode, Helius und Alchemy zur Berechnung dynamischer Gebühren auf der Solana-Blockchain.
- **Personalisierte Mining-Challenges**: Jede Mining-Session wird individuell optimiert, um faire Bedingungen für alle Miner zu gewährleisten.
- **Benachrichtigungen in Echtzeit**: Echtzeit-Updates über Mining-Fortschritte und Mining-Ergebnisse über WebSockets und andere Benachrichtigungssysteme.
- **Optimiert für Solana**: Der Server nutzt die Geschwindigkeit und Effizienz der Solana-Blockchain.

## Installation

### 1. Rust-Umgebung einrichten

Stelle sicher, dass Rust und die Solana-CLI installiert sind:

- [Rust Installation](https://www.rust-lang.org/tools/install)
- [Solana CLI Installation](https://docs.solana.com/cli/install-solana-cli-tools)

Klone das Repository:

```bash
git clone https://github.com/mistergrow/OreNetServer.git
cd OreNetServer
