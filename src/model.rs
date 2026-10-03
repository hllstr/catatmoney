use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Income,
    Expense,
    Transfer,
}

impl TransactionType {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Income => "Pemasukan",
            Self::Expense => "Pengeluaran",
            Self::Transfer => "Transfer Antar Akun",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletType {
    Bank,
    EWallet,
    Cash,
    CreditCard,
    Other,
}

impl WalletType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bank => "Rekening Bank",
            Self::EWallet => "E-Wallet",
            Self::Cash => "Uang Tunai",
            Self::CreditCard => "Kartu Kredit",
            Self::Other => "Lainnya",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    pub name: String,
    pub wallet_type: WalletType,
    pub initial_balance: f64,
}

pub fn default_wallet() -> String {
    "BCA".to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCategories {
    pub expense: Vec<String>,
    pub income: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub title: String,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub category: String,
    #[serde(default = "default_wallet")]
    pub wallet: String,
    #[serde(default)]
    pub to_wallet: Option<String>,
    #[serde(default)]
    pub admin_fee: Option<f64>,
    pub date: String, // YYYY-MM-DD
    pub time: String, // HH:mm:ss
    pub notes: String,
    pub attachment_name: Option<String>,
    pub attachment_size: Option<String>,
    pub attachment_data: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
}

pub const EXPENSE_CATEGORIES: &[&str] = &[
    "Makanan & Minuman",
    "Transportasi",
    "Belanja",
    "Tagihan",
    "Hiburan",
    "Kesehatan",
    "Lainnya",
];

pub const INCOME_CATEGORIES: &[&str] = &[
    "Gaji",
    "Freelance",
    "Investasi",
    "Hadiah",
    "Lainnya",
];

#[allow(dead_code)]
pub const STORAGE_KEY: &str = "catatmoney_transactions_v2";
#[allow(dead_code)]
pub const THEME_KEY: &str = "catatmoney_theme_mode";
#[allow(dead_code)]
pub const STORAGE_KEY_WALLETS: &str = "catatmoney_wallets_v1";
#[allow(dead_code)]
pub const STORAGE_KEY_CATEGORIES: &str = "catatmoney_categories_v1";
#[allow(dead_code)]
pub const STORAGE_KEY_PROFILE: &str = "catatmoney_profile_v1";
#[allow(dead_code)]
pub const STORAGE_KEY_BUDGETS: &str = "catatmoney_budgets_v2";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoryBudget {
    pub id: String,
    pub category: String,
    pub monthly_limit: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: String,
    pub is_onboarded: bool,
}



/// Format nominal ke format mata uang Rupiah Indonesia (contoh: Rp 50.000)
pub fn format_idr(amount: f64) -> String {
    let is_negative = amount < 0.0;
    let abs_val = amount.abs().round() as u64;
    let s = abs_val.to_string();
    let mut result = String::new();
    let bytes = s.as_bytes();
    let len = bytes.len();

    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push('.');
        }
        result.push(b as char);
    }

    if is_negative {
        format!("-Rp {}", result)
    } else {
        format!("Rp {}", result)
    }
}

/// Helper untuk memformat angka dengan pemisah ribuan titik (contoh: 50000 -> 50.000)
pub fn format_number_dots(val: u64) -> String {
    let s = val.to_string();
    let mut result = String::new();
    let bytes = s.as_bytes();
    let len = bytes.len();

    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push('.');
        }
        result.push(b as char);
    }
    result
}

/// Parsing input angka mentah dari user secara live ke nominal float dan string terformat ribuan
pub fn parse_input_idr(raw: &str) -> (f64, String) {
    let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return (0.0, String::new());
    }
    let num: u64 = digits.parse().unwrap_or(0);
    let formatted = format_number_dots(num);
    (num as f64, formatted)
}

/// Menghasilkan ID unik
pub fn generate_id() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(crypto) = window.crypto() {
                return crypto.random_uuid();
            }
        }
    }

    format!("trx_{}", js_sys::Date::now() as u64)
}

/// Mendapatkan tanggal hari ini dalam format YYYY-MM-DD
pub fn get_today_date() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let year = date.get_full_year();
        let month = date.get_month() + 1;
        let day = date.get_date();
        format!("{:04}-{:02}-{:02}", year, month, day)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "2026-10-02".to_string()
    }
}

/// Mendapatkan tanggal N hari yang lalu dalam format YYYY-MM-DD
pub fn get_date_days_ago(days: i32) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let ms = date.get_time() - (days as f64 * 86_400_000.0);
        let past = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms));
        format!(
            "{:04}-{:02}-{:02}",
            past.get_full_year(),
            past.get_month() + 1,
            past.get_date()
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = days;
        "2026-09-02".to_string()
    }
}

/// Mendapatkan tanggal hari Senin dari minggu berjalan dalam format YYYY-MM-DD
pub fn get_start_of_this_week() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let day_of_week = date.get_day() as i32; // 0 = Minggu, 1 = Senin, ..., 6 = Sabtu
        let diff_to_monday = if day_of_week == 0 { 6 } else { day_of_week - 1 };
        let ms = date.get_time() - (diff_to_monday as f64 * 86_400_000.0);
        let monday = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms));
        format!(
            "{:04}-{:02}-{:02}",
            monday.get_full_year(),
            monday.get_month() + 1,
            monday.get_date()
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "2026-09-28".to_string()
    }
}

/// Mendapatkan jumlah hari dalam bulan saat ini dan hari ke-berapa hari ini (current_day, total_days_in_month)
pub fn get_month_days_info() -> (u32, u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let year = date.get_full_year() as u32;
        let month = date.get_month() as i32; // 0-indexed (0 = Jan, 9 = Okt, dll.)
        let current_day = date.get_date() as u32;
        // Hari ke-0 dari bulan berikutnya adalah hari terakhir bulan ini
        let last_day_date = js_sys::Date::new_with_year_month_day(year, month + 1, 0);
        let total_days = last_day_date.get_date() as u32;
        (current_day, total_days)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        (3, 31)
    }
}



/// Mendapatkan waktu saat ini dengan jam, menit, dan detik (HH:mm:ss)
pub fn get_current_time() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let hours = date.get_hours();
        let minutes = date.get_minutes();
        let seconds = date.get_seconds();
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "14:30:45".to_string()
    }
}

/// Mendapatkan waktu saat ini dengan jam dan menit (HH:mm) untuk time/clock picker
pub fn get_current_time_hm() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let hours = date.get_hours();
        let minutes = date.get_minutes();
        format!("{:02}:{:02}", hours, minutes)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "14:30".to_string()
    }
}

/// Memuat profil pengguna dari LocalStorage browser
pub fn load_profile() -> Option<UserProfile> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(raw_json)) = storage.get_item(STORAGE_KEY_PROFILE) {
                    if let Ok(profile) = serde_json::from_str::<UserProfile>(&raw_json) {
                        return Some(profile);
                    }
                }
            }
        }
    }
    None
}

/// Menyimpan profil pengguna ke LocalStorage browser
pub fn save_profile(_profile: &UserProfile) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(raw_json) = serde_json::to_string(_profile) {
                    let _ = storage.set_item(STORAGE_KEY_PROFILE, &raw_json);
                }
            }
        }
    }
}

/// Menghapus seluruh data aplikasi (transaksi, sumber dana, kategori, dan profil)
pub fn reset_all_data() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item(STORAGE_KEY);
                let _ = storage.remove_item(STORAGE_KEY_WALLETS);
                let _ = storage.remove_item(STORAGE_KEY_CATEGORIES);
                let _ = storage.remove_item(STORAGE_KEY_PROFILE);
                let _ = storage.remove_item(STORAGE_KEY_BUDGETS);
                let _ = storage.remove_item("catatmoney_budgets_v1");
            }
        }
    }
}

/// Memuat transaksi dari LocalStorage browser
pub fn load_transactions() -> Vec<Transaction> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(raw_json)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(mut transactions) = serde_json::from_str::<Vec<Transaction>>(&raw_json) {
                        let mut needs_save = false;
                        for t in &mut transactions {
                            if let Some(ref d) = t.attachment_data {
                                if d.starts_with("data:image/svg+xml") && d != SAMPLE_RECEIPT_DATA_URI {
                                    t.attachment_data = Some(SAMPLE_RECEIPT_DATA_URI.to_string());
                                    needs_save = true;
                                }
                            }
                        }
                        if needs_save {
                            if let Ok(updated_raw) = serde_json::to_string(&transactions) {
                                let _ = storage.set_item(STORAGE_KEY, &updated_raw);
                            }
                        }
                        return transactions;
                    }
                }
            }
        }
    }

    vec![]
}


/// Menyimpan data transaksi ke LocalStorage browser
pub fn save_transactions(_transactions: &[Transaction]) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(raw_json) = serde_json::to_string(_transactions) {
                    let _ = storage.set_item(STORAGE_KEY, &raw_json);
                }
            }
        }
    }
}

/// Memuat preferensi tema dari LocalStorage
pub fn load_theme() -> ThemeMode {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(val)) = storage.get_item(THEME_KEY) {
                    if val == "light" {
                        return ThemeMode::Light;
                    }
                }
            }
        }
    }
    ThemeMode::Dark
}

/// Menyimpan preferensi tema ke LocalStorage
pub fn save_theme(_theme: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let val = match _theme {
                    ThemeMode::Dark => "dark",
                    ThemeMode::Light => "light",
                };
                let _ = storage.set_item(THEME_KEY, val);
            }
        }
    }
}

pub const SAMPLE_RECEIPT_DATA_URI: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0naHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmcnIHZpZXdCb3g9JzAgMCAzMjAgMjIwJyB3aWR0aD0nMzIwJyBoZWlnaHQ9JzIyMCc+PHJlY3Qgd2lkdGg9JzMyMCcgaGVpZ2h0PScyMjAnIHJ4PScxMCcgZmlsbD0nIzEyMTQxYycgc3Ryb2tlPScjMjcyNzJmJyBzdHJva2Utd2lkdGg9JzEuNScvPjx0ZXh0IHg9JzIwJyB5PSczMicgZmlsbD0nI2ZhZmFmYScgZm9udC1mYW1pbHk9J3NhbnMtc2VyaWYnIGZvbnQtc2l6ZT0nMTMnIGZvbnQtd2VpZ2h0PSc3MDAnPkdSQU5EIExVQ0tZIFNVUEVSU1RPUkU8L3RleHQ+PHRleHQgeD0nMjAnIHk9JzUwJyBmaWxsPScjNzE3MTdhJyBmb250LWZhbWlseT0nc2Fucy1zZXJpZicgZm9udC1zaXplPScxMCc+U1RSVUsgUkVTTUkgIzg4MjkxIOKAoiAwMSBPS1QgMjAyNiAxNzo0MjwvdGV4dD48bGluZSB4MT0nMjAnIHkxPSc2NCcgeDI9JzMwMCcgeTI9JzY0JyBzdHJva2U9JyMyNzI3MmYnIHN0cm9rZS1kYXNoYXJyYXk9JzQgMycvPjx0ZXh0IHg9JzIwJyB5PSc4OCcgZmlsbD0nI2ExYTFhYScgZm9udC1mYW1pbHk9J3NhbnMtc2VyaWYnIGZvbnQtc2l6ZT0nMTEnPkJhaGFuIFBhbmdhbiBTZWdhcjwvdGV4dD48dGV4dCB4PSczMDAnIHk9Jzg4JyB0ZXh0LWFuY2hvcj0nZW5kJyBmaWxsPScjZmFmYWZhJyBmb250LWZhbWlseT0nc2Fucy1zZXJpZicgZm9udC1zaXplPScxMSc+UnAgMjkwLjAwMDwvdGV4dD48dGV4dCB4PScyMCcgeT0nMTEwJyBmaWxsPScjYTFhMWFhJyBmb250LWZhbWlseT0nc2Fucy1zZXJpZicgZm9udC1zaXplPScxMSc+UGVybGVuZ2thcGFuIERhcHVyPC90ZXh0Pjx0ZXh0IHg9JzMwMCcgeT0nMTEwJyB0ZXh0LWFuY2hvcj0nZW5kJyBmaWxsPScjZmFmYWZhJyBmb250LWZhbWlseT0nc2Fucy1zZXJpZicgZm9udC1zaXplPScxMSc+UnAgMTg1LjAwMDwvdGV4dD48bGluZSB4MT0nMjAnIHkxPScxMzAnIHgyPSczMDAnIHkyPScxMzAnIHN0cm9rZT0nIzI3MjcyZicvPjx0ZXh0IHg9JzIwJyB5PScxNTUnIGZpbGw9JyNmYWZhZmEnIGZvbnQtZmFtaWx5PSdzYW5zLXNlcmlmJyBmb250LXNpemU9JzEyJyBmb250LXdlaWdodD0nNzAwJz5UT1RBTCBCQVlBUjwvdGV4dD48dGV4dCB4PSczMDAnIHk9JzE1NScgdGV4dC1hbmNob3I9J2VuZCcgZmlsbD0nIzEwYjk4MScgZm9udC1mYW1pbHk9J3NhbnMtc2VyaWYnIGZvbnQtc2l6ZT0nMTMnIGZvbnQtd2VpZ2h0PSc4MDAnPlJwIDQ3NS4wMDA8L3RleHQ+PHRleHQgeD0nMjAnIHk9JzE4NScgZmlsbD0nIzcxNzE3YScgZm9udC1mYW1pbHk9J3NhbnMtc2VyaWYnIGZvbnQtc2l6ZT0nMTAnPktBUlRVIERFQklUIEJDQSDigKkgT1RPUklTQVNJICMwOTkyNDE8L3RleHQ+PHRleHQgeD0nMjAnIHk9JzIwMCcgZmlsbD0nIzA1OTY2OScgZm9udC1mYW1pbHk9J3NhbnMtc2VyaWYnIGZvbnQtc2l6ZT0nMTAnIGZvbnQtd2VpZ2h0PSc2MDAnPlNUQVRVUzogTFVOQVMgJmFtcDsgVEVSVkVSSUZJS0FTSTwvdGV4dD48L3N2Zz4=";

/// Format tanggal ringkas (contoh: 2026-10-02 -> 02/10)
pub fn format_short_date(date_str: &str) -> String {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        format!("{}/{}", parts[2], parts[1])
    } else {
        date_str.to_string()
    }
}

#[allow(dead_code)]
pub fn get_default_wallets() -> Vec<Wallet> {
    vec![
        Wallet {
            id: "wallet_bca".to_string(),
            name: "BCA".to_string(),
            wallet_type: WalletType::Bank,
            initial_balance: 5000000.0,
        },
        Wallet {
            id: "wallet_gopay".to_string(),
            name: "GoPay".to_string(),
            wallet_type: WalletType::EWallet,
            initial_balance: 350000.0,
        },
        Wallet {
            id: "wallet_cash".to_string(),
            name: "Uang Tunai".to_string(),
            wallet_type: WalletType::Cash,
            initial_balance: 200000.0,
        },
    ]
}

pub fn load_wallets() -> Vec<Wallet> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(raw_json)) = storage.get_item(STORAGE_KEY_WALLETS) {
                    if let Ok(wallets) = serde_json::from_str::<Vec<Wallet>>(&raw_json) {
                        return wallets;
                    }
                }
            }
        }
    }
    vec![]
}

pub fn save_wallets(_wallets: &[Wallet]) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(raw_json) = serde_json::to_string(_wallets) {
                    let _ = storage.set_item(STORAGE_KEY_WALLETS, &raw_json);
                }
            }
        }
    }
}

pub fn get_default_categories() -> UserCategories {
    UserCategories {
        expense: EXPENSE_CATEGORIES.iter().map(|s| s.to_string()).collect(),
        income: INCOME_CATEGORIES.iter().map(|s| s.to_string()).collect(),
    }
}

pub fn load_categories() -> UserCategories {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(raw_json)) = storage.get_item(STORAGE_KEY_CATEGORIES) {
                    if let Ok(cats) = serde_json::from_str::<UserCategories>(&raw_json) {
                        return cats;
                    }
                }
            }
        }
    }
    get_default_categories()
}

pub fn save_categories(_categories: &UserCategories) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(raw_json) = serde_json::to_string(_categories) {
                    let _ = storage.set_item(STORAGE_KEY_CATEGORIES, &raw_json);
                }
            }
        }
    }
}

pub fn get_default_budgets() -> Vec<CategoryBudget> {
    vec![
        CategoryBudget {
            id: "budget_food".to_string(),
            category: "Makanan & Minuman".to_string(),
            monthly_limit: 2500000.0,
        },
        CategoryBudget {
            id: "budget_groceries".to_string(),
            category: "Belanja".to_string(),
            monthly_limit: 1500000.0,
        },
        CategoryBudget {
            id: "budget_transport".to_string(),
            category: "Transportasi".to_string(),
            monthly_limit: 800000.0,
        },
        CategoryBudget {
            id: "budget_bills".to_string(),
            category: "Tagihan".to_string(),
            monthly_limit: 1200000.0,
        },
    ]
}

pub fn load_budgets() -> Vec<CategoryBudget> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(raw_json)) = storage.get_item(STORAGE_KEY_BUDGETS) {
                    if let Ok(budgets) = serde_json::from_str::<Vec<CategoryBudget>>(&raw_json) {
                        return budgets;
                    }
                }
            }
        }
    }
    vec![]
}

pub fn save_budgets(_budgets: &[CategoryBudget]) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(raw_json) = serde_json::to_string(_budgets) {
                    let _ = storage.set_item(STORAGE_KEY_BUDGETS, &raw_json);
                }
            }
        }
    }
}

pub fn calculate_wallet_balance(wallet: &Wallet, transactions: &[Transaction]) -> f64 {

    let mut bal = wallet.initial_balance;
    let wallet_name = wallet.name.trim();

    for t in transactions {
        match t.transaction_type {
            TransactionType::Income => {
                if t.wallet.trim().eq_ignore_ascii_case(wallet_name) {
                    bal += t.amount;
                }
            }
            TransactionType::Expense => {
                if t.wallet.trim().eq_ignore_ascii_case(wallet_name) {
                    bal -= t.amount;
                }
            }
            TransactionType::Transfer => {
                // Akun Asal: berkurang sebesar nominal transfer + biaya admin (jika ada)
                if t.wallet.trim().eq_ignore_ascii_case(wallet_name) {
                    bal -= t.amount + t.admin_fee.unwrap_or(0.0);
                }
                // Akun Tujuan: bertambah sebesar nominal transfer
                if let Some(ref dest) = t.to_wallet {
                    if dest.trim().eq_ignore_ascii_case(wallet_name) {
                        bal += t.amount;
                    }
                }
            }
        }
    }
    bal
}

/// Data starter awal untuk mendemokan fitur baru (timestamp detik, lampiran, dan sumber dana)
#[allow(dead_code)]
fn get_starter_transactions() -> Vec<Transaction> {
    vec![
        Transaction {
            id: "trx_init_1".to_string(),
            title: "Gaji Pokok & Tunjangan".to_string(),
            amount: 8500000.0,
            transaction_type: TransactionType::Income,
            category: "Gaji".to_string(),
            wallet: "BCA".to_string(),
            to_wallet: None,
            admin_fee: None,
            date: "2026-10-01".to_string(),
            time: "09:15:20".to_string(),
            notes: "Transfer gaji awal bulan via BCA".to_string(),
            attachment_name: Some("slip_gaji_okt.pdf".to_string()),
            attachment_size: Some("142 KB".to_string()),
            attachment_data: None,
        },
        Transaction {
            id: "trx_init_2".to_string(),
            title: "Belanja Supermarket".to_string(),
            amount: 475000.0,
            transaction_type: TransactionType::Expense,
            category: "Belanja".to_string(),
            wallet: "BCA".to_string(),
            to_wallet: None,
            admin_fee: None,
            date: "2026-10-01".to_string(),
            time: "17:42:08".to_string(),
            notes: "Bahan pangan mingguan dan perlengkapan rumah".to_string(),
            attachment_name: Some("struk_belanja.jpg".to_string()),
            attachment_size: Some("1.2 MB".to_string()),
            attachment_data: Some(SAMPLE_RECEIPT_DATA_URI.to_string()),
        },
        Transaction {
            id: "trx_init_3".to_string(),
            title: "Makan Siang & Artisan Coffee".to_string(),
            amount: 68000.0,
            transaction_type: TransactionType::Expense,
            category: "Makanan & Minuman".to_string(),
            wallet: "GoPay".to_string(),
            to_wallet: None,
            admin_fee: None,
            date: "2026-10-02".to_string(),
            time: "12:28:44".to_string(),
            notes: "Makan siang dengan tim kantor".to_string(),
            attachment_name: None,
            attachment_size: None,
            attachment_data: None,
        },
        Transaction {
            id: "trx_init_4".to_string(),
            title: "Bonus Project Freelance".to_string(),
            amount: 1500000.0,
            transaction_type: TransactionType::Income,
            category: "Freelance".to_string(),
            wallet: "BCA".to_string(),
            to_wallet: None,
            admin_fee: None,
            date: "2026-10-02".to_string(),
            time: "15:10:30".to_string(),
            notes: "Pelunasan milestone web redesign".to_string(),
            attachment_name: Some("invoice_inv_042.pdf".to_string()),
            attachment_size: Some("88 KB".to_string()),
            attachment_data: None,
        },
        Transaction {
            id: "trx_init_5".to_string(),
            title: "Top Up GoPay dari BCA".to_string(),
            amount: 250000.0,
            transaction_type: TransactionType::Transfer,
            category: "Transfer Internal".to_string(),
            wallet: "BCA".to_string(),
            to_wallet: Some("GoPay".to_string()),
            admin_fee: Some(1000.0),
            date: "2026-10-02".to_string(),
            time: "18:20:15".to_string(),
            notes: "Isi saldo e-wallet untuk transportasi & makan".to_string(),
            attachment_name: None,
            attachment_size: None,
            attachment_data: None,
        },
    ]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatatMoneyBackup {
    pub version: u32,
    pub app: String,
    pub exported_at: String,
    #[serde(default)]
    pub profile: Option<UserProfile>,
    pub transactions: Vec<Transaction>,
    pub wallets: Vec<Wallet>,
    pub categories: UserCategories,
}

pub fn create_backup(
    profile: Option<&UserProfile>,
    transactions: &[Transaction],
    wallets: &[Wallet],
    categories: &UserCategories,
) -> CatatMoneyBackup {
    CatatMoneyBackup {
        version: 1,
        app: "CatatMoney".to_string(),
        exported_at: format!("{} {} WIB", get_today_date(), get_current_time()),
        profile: profile.cloned(),
        transactions: transactions.to_vec(),
        wallets: wallets.to_vec(),
        categories: categories.clone(),
    }
}

pub fn trigger_json_download(filename: &str, content: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Ok(blob) = web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(content))) {
                    if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                        if let Ok(el) = doc.create_element("a") {
                            let _ = el.set_attribute("href", &url);
                            let _ = el.set_attribute("download", filename);
                            let _ = el.set_attribute("style", "display: none;");
                            if let Some(body) = doc.body() {
                                let _ = body.append_child(&el);
                                if let Ok(anchor) = el.clone().dyn_into::<web_sys::HtmlElement>() {
                                    anchor.click();
                                }
                                let _ = body.remove_child(&el);
                            }
                            let _ = web_sys::Url::revoke_object_url(&url);
                        }
                    }
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (filename, content);
    }
}

