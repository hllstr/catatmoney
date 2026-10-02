# CatatMoney Design System & Specification (DESIGN.md)

> **Design Philosophy**: *"Monochrome Swiss FinTech"* — Elegan, tajam, presisi tinggi, dan bebas dari stereotip AI generik (*Anti-AI Slop*). Mendukung dual-mode: **Light Mode** & **Dark Mode**.

---

## 1. Prinsip Utama (Non-Negotiable)

1. **ZERO EMOJIS**:
   - Dilarang keras menggunakan karakter emoji di seluruh bagian aplikasi.
   - Semua representasi visual wajib menggunakan vektor SVG presisi (1.5px / 1.75px stroke, clean Lucide style) di `src/components/icons.rs`.
2. **MONOCHROME PALETTE DUAL-MODE**:
   - Desain berbasis palet monokromatik murni (Black, White, Zinc, Neutral Grays).
   - Mendukung **Light Mode** (latar putih bersih/off-white dengan kontras teks hitam pekat) dan **Dark Mode** (latar obsidian/zinc pekat dengan kontras teks putih tajam).
3. **TAILWIND CSS & DESIGN TOKENS**:
   - Menggunakan utility Tailwind CSS dikombinasikan dengan token CSS variabel untuk transisi mulus dan styling granular.
4. **TABULAR FIGURES FOR FINANCIAL NUMBERS**:
   - Semua angka keuangan wajib menggunakan class `.tabular-nums` (`font-variant-numeric: tabular-nums; font-feature-settings: "tnum";`).
5. **BOTTOM NAVIGATION DOCK & ACTION BUTTON**:
   - Navigasi utama diletakkan di bagian bawah layar (Bottom Navigation Bar) lengkap dengan tombol aksi utama **(+) Tambah Transaksi** di posisi tengah mengambang.

---

## 2. Tipografi (Google Fonts)

*   **Primary Font**: [Plus Jakarta Sans](https://fonts.google.com/specimen/Plus+Jakarta+Sans)
*   **Bobot**:
    *   `400` (Regular): Paragraf, detail keterangan, placeholder.
    *   `500` (Medium): Label input, metadata, tanggal.
    *   `600` (Semi-Bold): Nilai mata uang, judul kategori, tombol tab.
    *   `700` / `800` (Bold): Total saldo utama, judul metrik.

---

## 3. Palet Monokromatik (Light & Dark Tokens)

```css
/* Light Mode (Default Root) */
:root, html.light {
  --bg-app: #fcfcfd;
  --bg-surface: #ffffff;
  --bg-surface-subtle: #f4f4f6;
  --bg-input: #f8f8fa;
  --border-subtle: #e4e4e7;
  --border-strong: #d4d4d8;
  --text-primary: #09090b;
  --text-secondary: #52525b;
  --text-muted: #71717a;
  --accent-contrast: #09090b;
  --positive: #059669;
  --negative: #e11d48;
}

/* Dark Mode */
html.dark {
  --bg-app: #09090b;
  --bg-surface: #121215;
  --bg-surface-subtle: #18181b;
  --bg-input: #141417;
  --border-subtle: #27272a;
  --border-strong: #3f3f46;
  --text-primary: #fafafa;
  --text-secondary: #a1a1aa;
  --text-muted: #71717a;
  --accent-contrast: #fafafa;
  --positive: #10b981;
  --negative: #f43f5e;
}
```

---

## 4. Komponen Arsitektur Baru

1. **Dashboard Eksekutif**:
   - Hero Saldo Bersih, Inflow, Outflow, Net Ratio.
   - **Grafik Tren Arus Kas**: Visualisasi grafik batang/area SVG harian 7 hari terakhir (bebas library JS luar, ringan & responsif).
   - Breakdown pengeluaran per kategori.
2. **Kalender Keuangan (Cashflow Calendar)**:
   - Tampilan grid bulanan dengan indikator pemasukan/pengeluaran per hari.
   - Interaktif: Klik tanggal untuk memeriksa transaksi pada hari tersebut.
3. **Bottom Navigation Dock**:
   - Dock mengambang di bawah: Tab **Dashboard**, Tab **Kalender**, Tab **Riwayat**, dan tombol tengah **(+) Tambah**.
4. **Modal Transaksi Detail**:
   - Timestamp detail hingga detik (`YYYY-MM-DD HH:mm:ss`).
   - Lampiran bukti/struk transaksi (nama file, tipe, status lampiran).
