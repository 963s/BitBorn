BitBorn (BTB) — Open Source Starter Repo

Overview
--------
This repository is the public, open‑source starter kit for the BitBorn project.
It includes:
- A minimal Rust prototype node (`bitborn-node`) with placeholders for networking, PoW and privacy modules.
- Open‑source governance and security docs (CONTRIBUTING, CODE_OF_CONDUCT, SECURITY).
- GitHub Actions CI workflow to check the Rust project builds.

Quick Start
-----------
1) Install Rust: https://www.rust-lang.org/tools/install
2) Build:
   cargo build -p bitborn-node
3) Run:
   cargo run -p bitborn-node

Project Layout
--------------
- bitborn-node/          Minimal Rust node prototype
- .github/workflows/     CI pipelines
- docs/                  Documentation placeholders
- GOVERNANCE.md          BBIPs governance model
- CONTRIBUTING.md        How to contribute
- CODE_OF_CONDUCT.md     Community conduct guidelines
- SECURITY.md            How to report security issues
- LICENSE                Apache-2.0 for code
- LICENSE-Whitepaper.md  CC BY 4.0 for docs/whitepaper

Licensing
---------
- Code: Apache-2.0 (permissive, patent grant)
- Docs (whitepaper): CC BY 4.0 (attribution)

Whitepaper Links
----------------
Add your IPFS CIDs in docs/WHITEPAPER_V2_LINKS.md.
