# CatatMoney

[![Build Status](https://img.shields.io/github/actions/workflow/status/hllstr/catatmoney/deploy.yml?branch=main&style=flat-square)](https://github.com/hllstr/catatmoney/actions)
[![Rust](https://img.shields.io/badge/rust-2024_edition-dea584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/dioxus-v0.7.10-e05338?style=flat-square)](https://dioxuslabs.com/)
[![WebAssembly](https://img.shields.io/badge/target-wasm32-654ff0?style=flat-square&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![PWA Ready](https://img.shields.io/badge/PWA-offline--first-0052cc?style=flat-square)](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps)

**CatatMoney** is a privacy-focused personal cashflow and financial tracking web application built with Rust, Dioxus 0.7, and WebAssembly. Designed around Swiss financial typography, multi-account liquidity management, procedural Web Audio sound feedback, and offline-first architecture, it runs entirely client-side without external database or cloud requirements.

Live Application: [https://hllstr.github.io/catatmoney/](https://hllstr.github.io/catatmoney/)

---

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Architecture](#architecture)
- [Technology Stack](#technology-stack)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation & Local Development](#installation--local-development)
  - [Building for Production](#building-for-production)
- [Continuous Deployment](#continuous-deployment)
- [Privacy & Security](#privacy--security)
- [License](#license)

---

## Overview

Modern personal finance tracking often requires recurring cloud subscriptions, remote tracking scripts, and fragmented interfaces. CatatMoney approaches financial tracking from an engineering perspective:

1. **Deterministic Client-Side Execution**: All ledger aggregation, cashflow metrics, and storage persistence execute directly in the browser.
2. **High-Density Typography**: Clean monochrome visual hierarchy using Google Fonts Plus Jakarta Sans and tabular figures (`tabular-nums`) to ensure strict vertical alignment of currencies and numbers.
3. **Multi-Source Cashflow**: Full visibility into liquid balances across physical wallets, bank accounts, and digital payment providers.
4. **Full Offline Capabilities**: Built as a Progressive Web App (PWA) with automatic Service Worker asset pre-caching.

---

## Key Features

### Executive Cashflow Dashboard
- Immediate overview of Net Balance, Total Income, and Total Expenses.
- Dynamic 7-day cashflow trend visualizations rendered as pure vector SVGs.
- Horizontal wallet carousel displaying live liquidity per source account.
- Category expense distribution with proportional percentage metrics.

### Multi-Account & Wallet Flow Tracking
- Support for multiple account types: Bank Accounts, E-Wallets, Cash, Credit Cards, and Custom Pos.
- Inter-account balance transfers with dedicated administrative fee tracking.
- Initial balance reconciliation and net asset trajectory tracking.

### Date-Grouped Transaction Ledger
- Chronological transaction organization with automated daily subtotals.
- Granular search by keyword and filtering by transaction category.
- Transaction attachments support for local receipt photos and invoice documents.
- Detailed modal inspection with inline image preview.

### Visual Analytics & Liquidity Curves
- **SVG Donut Allocation**: Dynamic ring charts for income and expense category distributions.
- **Cashflow Bar Charts**: Bi-directional velocity comparison across customizable periods (This Month, 30 Days, This Year, All Time).
- **Cumulative Running Balance Curve**: Smooth cubic-bezier spline mapping cumulative liquid balance over time, with automated period ATH (All-Time High) and ATL (All-Time Low) markers.

### Budget Management & Savings Goals
- Category-level monthly expense ceilings with real-time health indicators (Safe, Warning, Exceeded).
- Target savings vaults with progress meters, target deadlines, and contribution history logs.

### Micro-Interactions & Sensory Feedback
- **Procedural Sound Engine**: Native Web Audio API sound generator delivering distinct synthetic acoustic signatures for cashflow events (metallic chime for income, paper friction for expenses, harmonized beam for transfers, tactile clicks for buttons) with zero external audio assets.
- **Fluid Motion Design**: 220ms GPU-accelerated dialog pop-ins, backdrop blur transitions, and mobile slide-down bottom sheets.
- **Privacy Mode**: Instant one-click masking of all monetary values (`Rp ••••••`) for shared or public environments.

### Curated Theme Engine
9 built-in color schemes selectable with one click from the global navigation header:
- Obsidian Dark (Default)
- Clean Light
- Tokyo Night
- Rosé Pine
- Sakura Blossom
- Catppuccin Latte
- Catppuccin Frappé
- Catppuccin Macchiato
- Catppuccin Mocha

---

## Architecture

```
catatmoney/
├── assets/
│   ├── icon.svg             # Vector application logo
│   ├── style.css            # Custom styling and GPU keyframe animations
│   └── tailwind.css         # Compiled Tailwind utility classes
├── scripts/
│   └── generate-sw.js       # Pre-cache manifest generator for Service Worker
├── src/
│   ├── components/
│   │   ├── ai_copilot.rs     # Optional Gemini AI financial assistant
│   │   ├── analytics.rs      # Donut allocations, bar charts, and liquidity curve
│   │   ├── bottom_nav.rs     # Floating navigation dock
│   │   ├── budget.rs         # Category budget tracking component
│   │   ├── calendar.rs       # Cashflow calendar with daily status indicators
│   │   ├── category_modal.rs # Category create/edit modal dialog
│   │   ├── chart.rs          # Dashboard SVG trend chart
│   │   ├── confirm_modal.rs  # Deletion and reset verification dialog
│   │   ├── dashboard.rs      # Executive summary and liquidity overview
│   │   ├── detail_modal.rs   # Transaction inspection and receipt viewer
│   │   ├── form_modal.rs     # Transaction input modal dialog
│   │   ├── history.rs        # Date-grouped transaction ledger
│   │   ├── icons.rs          # Modular handcrafted SVG vectors
│   │   ├── management.rs     # Source accounts, categories, and backup hub
│   │   ├── profile_modal.rs  # User profile edit modal dialog
│   │   ├── savings.rs        # Savings goals and contribution ledger
│   │   ├── theme_dropdown.rs # Global header 1-click theme picker
│   │   ├── toast.rs          # Floating notifications with spring physics
│   │   └── wallet_modal.rs   # Wallet create/edit modal dialog
│   ├── audio.rs             # Procedural Web Audio API sound synthesizer
│   ├── gemini.rs            # Client-side Google Gemini REST integration
│   ├── model.rs             # Core data structures, enums, and currency parsers
│   ├── storage.rs           # LocalStorage synchronization layer
│   └── main.rs              # Application root and reactive state orchestrator
├── Cargo.toml               # Rust package manifest
├── Dioxus.toml              # Dioxus build configuration
└── package.json             # Tailwind CSS build scripts
```

---

## Technology Stack

- **Core Framework**: [Dioxus 0.7](https://dioxuslabs.com/) (Rust WebAssembly)
- **Language**: [Rust](https://www.rust-lang.org/) (2024 Edition)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) with CSS Custom Properties
- **Typography**: [Plus Jakarta Sans](https://fonts.google.com/specimen/Plus+Jakarta+Sans) via Google Fonts
- **Audio Engine**: Synthesized Procedural Web Audio API (`web-sys::AudioContext`)
- **Persistence**: Browser `LocalStorage` via `web-sys::Storage`
- **PWA Runtime**: Custom Service Worker (`sw.js`) with Cache Storage pre-caching

---

## Getting Started

### Prerequisites

Ensure you have the Rust toolchain and required build tools installed:

1. Install Rust via [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Add the WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install the Dioxus CLI:
   ```bash
   cargo install dioxus-cli --locked
   ```

4. Install Node.js (v18+) for compiling Tailwind CSS.

### Installation & Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/hllstr/catatmoney.git
   cd catatmoney
   ```

2. Install development dependencies:
   ```bash
   npm install
   ```

3. Build the Tailwind stylesheet:
   ```bash
   npm run build:css
   ```

4. Start the Dioxus development server:
   ```bash
   dx serve
   ```

5. Open your browser at `http://127.0.0.1:8080/`.

### Building for Production

To create an optimized WebAssembly release package:

```bash
npm run build:css
dx build --release --web
```

Compiled static assets will be output to:
```
target/dx/catatmoney/release/web/public/
```

---

## Continuous Deployment

This repository includes an automated GitHub Actions deployment workflow located at `.github/workflows/deploy.yml`.

Every push to the `main` branch:
1. Compiles CSS assets via Tailwind CLI.
2. Builds the optimized WebAssembly binary (`wasm-opt` enabled).
3. Automatically generates the offline Service Worker cache manifest.
4. Deploys the static site to GitHub Pages.

---

## Privacy & Security

- **Zero Remote Tracking**: CatatMoney does not include analytics, telemetry, remote database connections, or tracking pixels.
- **Client-Side Data Ownership**: All financial records, receipts, and custom categories remain strictly within your device's browser storage.
- **Backup & Portability**: Users can export full JSON backups at any time from the Management tab to ensure data portability across devices.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
