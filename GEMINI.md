# GEMINI.md - Aturan & Kontrak Kerja AI Antigravity untuk CatatMoney

Dokumen ini berisi aturan permanen dan panduan perilaku AI saat mengembangkan proyek **CatatMoney**. Setiap perubahan kode antarmuka dan logika wajib mematuhi aturan berikut.

---

## 1. Aturan Wajib Desain & Estetika (Anti-AI Slop)

1. **LARANGAN MUTLAK EMOJI**:
   - **JANGAN PERNAH** menggunakan karakter Emoji (seperti 💰, 💳, 📈, 📉, ➕, 🗑️, 🍜, dll.) di bagian mana pun dalam kode, UI, label, teks, atau placeholder.
   - Semua elemen visual wajib menggunakan **vektor SVG presisi** (stroke 1.5px / 1.75px, clean Lucide style) yang didefinisikan secara modular di `src/components/icons.rs`.

2. **WAJIB MENGIKUTI DESIGN.md**:
   - Selalu jadikan [DESIGN.md](file:///workspaces/catatmoney/DESIGN.md) sebagai acuan tunggal untuk warna, tipografi, grid, dan komponen.
   - Jangan pernah menambahkan gaya gradien ungu/indigo template AI yang generik.
   - Gunakan pendekatan *Obsidian Dark Minimalist* dengan batas 1px tipis (*hairline borders*) dan kontras tipografi tajam.

3. **STANDAR TIPOGRAFI (GOOGLE FONTS)**:
   - Selalu gunakan font resmi Google Fonts: **Plus Jakarta Sans**.
   - Setiap elemen yang menampilkan angka keuangan / mata uang **wajib** menyertakan class/style tabular numbers (`font-variant-numeric: tabular-nums; font-feature-settings: "tnum";`) agar angka sejajar rapi.

4. **DATA DENSITY & CRAFTSMANSHIP**:
   - Desain harus berorientasi pada fungsionalitas dan kenyamanan pembacaan data keuangan harian ala Copilot Money dan Linear.
   - Hindari card berjarak terlalu jauh dan bayangan melayang (*floaty blur shadows*).

---

## 2. Standar Kode & Arsitektur Dioxus 0.7

1. **Komponen Modular**:
   - Letakkan komponen di `src/components/`.
   - Ikon SVG diisolasi di `src/components/icons.rs` agar bisa dipakai ulang dengan mudah.
2. **State Reaktif**:
   - Gunakan signal Dioxus 0.7 (`use_signal`).
   - Sinkronisasi perubahan ke LocalStorage secara efisien tanpa blocking main thread.
3. **Kualitas Kompilasi Rust**:
   - Setiap perubahan kode harus bebas dari warning dan error kompilasi (`cargo check`).
   - Pastikan kode berjalan dengan baik di target WebAssembly (`dx build --web`).
