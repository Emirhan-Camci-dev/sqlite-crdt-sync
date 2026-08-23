```markdown
# sqlite-crdt-sync

> Offline-First SQLite with P2P CRDT & Transparent Encryption.

sqlite-crdt-sync is a lightweight, zero-data-loss SQLite extension built for edge environments like mining, remote logistics, and aviation. It enables devices to seamlessly synchronize local SQLite databases over P2P mesh networks (Bluetooth LE, mDNS) without requiring an internet connection.

## Quickstart (3-Lines)

Integrate `sqlite-crdt-sync` into your app and turn any local database into a P2P distributed mesh instantly with **sub-5ms** sync latency overhead:

```rust
use sqlite_crdt_sync::Engine;

// 1. Wrap your SQLite connection
let mut db = Engine::attach("app.db", b"my-encryption-key-32b");

// 2. Start the background P2P daemon on a random port
db.start_p2p_mesh("0.0.0.0:0").await;

// 3. Just write SQL as usual. It syncs in the background!
db.execute("INSERT INTO users (name) VALUES ('Alice')")?;

```

## Features

* **Transparent Encryption**: Page-level ChaCha20-Poly1305 encryption via custom SQLite VFS.
* **Offline Mesh P2P**: Devices discover each other via mDNS and sync securely using libp2p and the Noise Protocol.
* **CRDT Sync Engine**: Deterministic, Last-Write-Wins (LWW) conflict resolution using Hybrid Logical Clocks (HLC).
* **Zero Lock-In**: It’s just SQLite. Your application queries and inserts data as normal.

---

## Community vs. Enterprise Edition

This project is built on a Dual-Licensed Open-Core model.

| Feature | Community (AGPLv3) | Pro / Enterprise (Commercial) |
| --- | --- | --- |
| P2P Mesh Sync | ✅ Yes (mDNS / Local) | ✅ Yes + Cloud Relays |
| CRDT Engine | ✅ Yes | ✅ Yes |
| Transparent Encryption | ✅ Yes (Software) | ✅ Yes + HSM/Keystore |
| Role-Based Access Control | ❌ No | ✅ Yes (Cryptographic Filtering) |
| License | **AGPLv3** (Open Source) | **Commercial** (Seat-based) |
| Support | Community | Priority Email & Slack |

### Get Pro / Commercial License

The Pro edition is designed for mission-critical enterprise environments. It provides enterprise-grade compliance, hardware keystore integration, and operates with an offline, cryptographically signed License Key (Ed25519).

👉 **[Purchase a Commercial License on Polar.sh](https://buy.polar.sh/polar_cl_gIptL8M5prBYVEQTIw3tJAjcttdt2d7OMnzQP4dfFap)**

*Support sustainable open-source development!*

```[cite: 1, 3]

```
