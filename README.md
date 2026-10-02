# CatatMoney

> **Financial Cashflow & Expense Intelligence**  
> Aplikasi pencatatan keuangan pribadi bergaya *Monochrome Swiss FinTech* — cepat, privat, tanpa emoji, dan 100% offline-first.

---

## ✨ Filosofi Desain (*Craftsmanship & Anti-AI Slop*)

CatatMoney dirancang untuk pengguna yang mengutamakan kecepatan, ketajaman data, dan kesederhanaan tanpa distraksi:
- **Zero Emojis**: Sepenuhnya menggunakan vektor SVG presisi tinggi (*clean Lucide style*, stroke 1.5px/1.75px) yang konsisten dan modular.
- **Monochrome Swiss FinTech**: Palet monokromatik kontras tinggi dengan aksen halus, mendukung **Dark Mode (Obsidian)** dan **Light Mode (Clean Paper)**.
- **Tabular Financial Figures**: Seluruh angka keuangan diformat menggunakan angka tabular (`font-variant-numeric: tabular-nums;`) agar tersusun sejajar rapi.
- **Mobile-First Ergonomics**: Dioptimalkan secara mendalam untuk ponsel cerdas (Floating bottom dock adaptif, carousel sumber dana horizontal, slide-up bottom sheet, dan grid seimbang).
- **100% Offline & Privat**: Seluruh data tersimpan secara lokal di browser (`LocalStorage`) tanpa pelacak, analitik pihak ketiga, atau pengumpulan data eksternal.

---

## 🚀 Fitur Utama

1. **Dashboard Eksekutif**:
   - Ringkasan Total Saldo Bersih, Total Pemasukan, dan Total Pengeluaran.
   - Grafik tren arus kas 7 hari terakhir berbasis SVG vektor interaktif.
   - Carousel akun sumber dana horizontal dengan saldo real-time.
   - Breakdown persentase pengeluaran berdasarkan kategori.

2. **Kalender Arus Kas (*Cashflow Calendar*)**:
   - Tampilan visual bulanan dengan indikator warna titik dot harian (hijau pemasukan, merah pengeluaran).
   - Klik tanggal mana pun untuk memeriksa detail transaksi harian secara instan.

3. **Riwayat Transaksi Presisi**:
   - Pencarian cepat dan penyaringan (*filter*) berdasarkan jenis transaksi (Semua, Pengeluaran, Pemasukan).
   - Label badge akun (`[BCA]`, `[GoPay]`) dan kategori yang terlindungi dari pemotongan teks.
   - Indikator lampiran struk/dokumen transaksi.

4. **Detail Transaksi & Pratinjau Struk**:
   - Modal rincian dengan grid metadata simetris 2x2 (`Kategori`, `Sumber Dana`, `Tanggal`, `Waktu Pencatatan detik`).
   - Pratinjau visual struk belanja (gambar Base64 SVG/PNG/JPG) atau berkas dokumen lokal.

5. **Pusat Manajemen Data (Kelola)**:
   - Tambah & kelola akun Sumber Dana (`Rekening Bank`, `E-Wallet`, `Uang Tunai`, `Kartu Kredit`, `Lainnya`).
   - Tambah & kelola kategori kustom pemasukan dan pengeluaran.
   - **Cadangan Data (Backup & Restore JSON)**: Ekspor seluruh arsip transaksi dan pengaturan ke berkas `.json`, serta pulihkan data secara lokal kapan saja.

6. **Progressive Web App (PWA)**:
   - Dapat di-install langsung di layar utama smartphone Android & iPhone (*Add to Home Screen*) tanpa melalui App Store/Play Store, berjalan dalam mode layar penuh (*standalone*).

---

## 🛠️ Tech Stack

- **Framework**: [Dioxus 0.7](https://dioxuslabs.com/) (Rust WebAssembly)
- **Bahasa**: [Rust](https://www.rust-lang.org/) (Edition 2024)
- **Styling**: Tailwind CSS + Custom CSS Variables Design Tokens
- **Tipografi**: Google Fonts ([Plus Jakarta Sans](https://fonts.google.com/specimen/Plus+Jakarta+Sans))
- **Ikon**: Modular Handcrafted SVG Icons (Lucide-inspired)
- **Penyimpanan**: Browser LocalStorage via `web-sys`

---

## 💻 Menjalankan Secara Lokal

### Prasyarat:
1. Pasang [Rust & Cargo](https://rustup.rs/)
2. Tambahkan target WebAssembly:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Pasang Dioxus CLI:
   ```bash
   cargo install dioxus-cli --locked
   ```

### Menjalankan Server Pengembangan:
```bash
dx serve
```
Buka peramban di `http://127.0.0.1:8080/`.

### Mengompilasi untuk Rilis Web:
```bash
dx build --release --web
```
Hasil kompilasi siap saji akan berada di folder `target/dx/catatmoney/release/web/public/`.

---

## 📦 Deployment ke GitHub Pages

Repository ini telah dilengkapi dengan workflow GitHub Actions otomatis di `.github/workflows/deploy.yml`.

Setiap commit baru yang di-push ke branch `main` akan otomatis dikompilasi dan dideploy ke GitHub Pages:
1. Buka tab **Settings** > **Pages** di repository GitHub Anda.
2. Di bagian **Source**, pilih **GitHub Actions**.
3. Akses aplikasi CatatMoney Anda di `https://<username>.github.io/catatmoney/`.

---

## 📄 Lisensi

Didistribusikan di bawah Lisensi MIT. Bebas digunakan dan dimodifikasi untuk kebutuhan personal maupun komersial.
