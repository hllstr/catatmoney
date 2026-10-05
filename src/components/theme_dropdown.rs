use dioxus::prelude::*;
use crate::model::ThemeMode;
use crate::components::icons::{IconMoon, IconSun, IconSparkles, IconChevronDown, IconCheck};

struct ThemeOption {
    mode: ThemeMode,
    label: &'static str,
    dot_bg: &'static str,
    dot_surface: &'static str,
    dot_accent: &'static str,
    border_style: &'static str,
}

const STANDARD_THEMES: &[ThemeOption] = &[
    ThemeOption {
        mode: ThemeMode::Dark,
        label: "Obsidian Dark",
        dot_bg: "#09090b",
        dot_surface: "#18181c",
        dot_accent: "#fafafa",
        border_style: "border: 1px solid #3f3f46;",
    },
    ThemeOption {
        mode: ThemeMode::Light,
        label: "Clean Light",
        dot_bg: "#fcfcfd",
        dot_surface: "#ffffff",
        dot_accent: "#09090b",
        border_style: "border: 1px solid #cbd5e1;",
    },
    ThemeOption {
        mode: ThemeMode::TokyoNight,
        label: "Tokyo Night",
        dot_bg: "#16161e",
        dot_surface: "#1a1b26",
        dot_accent: "#7aa2f7",
        border_style: "border: 1px solid #3b4261;",
    },
    ThemeOption {
        mode: ThemeMode::RosePine,
        label: "Rosé Pine",
        dot_bg: "#191724",
        dot_surface: "#1f1d2e",
        dot_accent: "#ebbcba",
        border_style: "border: 1px solid #403d52;",
    },
    ThemeOption {
        mode: ThemeMode::SakuraBlossom,
        label: "Sakura Blossom",
        dot_bg: "#fff5f7",
        dot_surface: "#ffffff",
        dot_accent: "#e06c88",
        border_style: "border: 1px solid #e8b7c3;",
    },
];

const CATPPUCCIN_THEMES: &[ThemeOption] = &[
    ThemeOption {
        mode: ThemeMode::CatppuccinLatte,
        label: "Catppuccin Latte",
        dot_bg: "#eff1f5",
        dot_surface: "#ffffff",
        dot_accent: "#8839ef",
        border_style: "border: 1px solid #bcc0cc;",
    },
    ThemeOption {
        mode: ThemeMode::CatppuccinFrappe,
        label: "Catppuccin Frappé",
        dot_bg: "#292c3c",
        dot_surface: "#303446",
        dot_accent: "#ca9ee6",
        border_style: "border: 1px solid #51576d;",
    },
    ThemeOption {
        mode: ThemeMode::CatppuccinMacchiato,
        label: "Catppuccin Macchiato",
        dot_bg: "#1e2030",
        dot_surface: "#24273a",
        dot_accent: "#c6a0f6",
        border_style: "border: 1px solid #494d64;",
    },
    ThemeOption {
        mode: ThemeMode::CatppuccinMocha,
        label: "Catppuccin Mocha",
        dot_bg: "#181825",
        dot_surface: "#1e1e2e",
        dot_accent: "#cba6f7",
        border_style: "border: 1px solid #45475a;",
    },
];

fn get_theme_display(theme: ThemeMode) -> (&'static str, &'static str) {
    match theme {
        ThemeMode::Dark => ("Obsidian Dark", "moon"),
        ThemeMode::Light => ("Clean Light", "sun"),
        ThemeMode::TokyoNight => ("Tokyo Night", "sparkles"),
        ThemeMode::RosePine => ("Rosé Pine", "sparkles"),
        ThemeMode::SakuraBlossom => ("Sakura Blossom", "sun"),
        ThemeMode::CatppuccinLatte => ("Catppuccin Latte", "sun"),
        ThemeMode::CatppuccinFrappe => ("Catppuccin Frappé", "sparkles"),
        ThemeMode::CatppuccinMacchiato => ("Catppuccin Macchiato", "sparkles"),
        ThemeMode::CatppuccinMocha => ("Catppuccin Mocha", "sparkles"),
    }
}

/// Komponen pemilih tema bergaya dropdown popover yang elegan
#[component]
pub fn ThemeSelectorDropdown(
    current_theme: ThemeMode,
    on_change_theme: EventHandler<ThemeMode>,
    #[props(default = false)] align_right: bool,
) -> Element {
    let mut is_open = use_signal(|| false);
    let (label, icon_type) = get_theme_display(current_theme);

    rsx! {
        div { class: "relative inline-block text-left",
            button {
                r#type: "button",
                class: "theme-toggle-btn",
                onclick: move |_| {
                    let curr = *is_open.read();
                    is_open.set(!curr);
                },
                span { class: "flex items-center gap-1.5",
                    match icon_type {
                        "sun" => rsx! { IconSun { size: "14" } },
                        "moon" => rsx! { IconMoon { size: "14" } },
                        _ => rsx! { IconSparkles { size: "14" } },
                    }
                    span { class: "font-semibold", "{label}" }
                }
                span {
                    class: if *is_open.read() { "transition-transform duration-200 rotate-180 text-[var(--text-muted)]" } else { "transition-transform duration-200 text-[var(--text-muted)]" },
                    IconChevronDown { size: "12" }
                }
            }

            if *is_open.read() {
                // Backdrop transparan untuk menutup saat klik di luar
                div {
                    class: "fixed inset-0 z-40 cursor-default",
                    onclick: move |_| is_open.set(false),
                }

                div {
                    class: if align_right { "theme-dropdown-menu right-0" } else { "theme-dropdown-menu left-0" },
                    div { class: "theme-dropdown-header",
                        "Pilih Tema Tampilan"
                    }

                    // Grup 1: Standar & Kota
                    div { class: "px-2 py-1 text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]",
                        "Koleksi Standar & Kota"
                    }
                    for opt in STANDARD_THEMES {
                        {
                            let is_active = current_theme == opt.mode;
                            let mode = opt.mode;
                            rsx! {
                                button {
                                    key: "{opt.label}",
                                    r#type: "button",
                                    class: if is_active { "theme-dropdown-item active" } else { "theme-dropdown-item" },
                                    onclick: move |_| {
                                        on_change_theme.call(mode);
                                        is_open.set(false);
                                    },
                                    div { class: "theme-dropdown-dots",
                                        div {
                                            class: "theme-dropdown-dot",
                                            style: "background-color: {opt.dot_bg}; {opt.border_style}",
                                        }
                                        div {
                                            class: "theme-dropdown-dot",
                                            style: "background-color: {opt.dot_surface}; {opt.border_style}",
                                        }
                                        div {
                                            class: "theme-dropdown-dot",
                                            style: "background-color: {opt.dot_accent};",
                                        }
                                    }
                                    span { class: "flex-1 truncate text-left", "{opt.label}" }
                                    if is_active {
                                        span { class: "text-[var(--accent)] shrink-0 ml-auto",
                                            IconCheck { size: "13" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Grup 2: Catppuccin Series
                    div { class: "px-2 pt-2 pb-1 text-[10px] font-bold uppercase tracking-wider text-[var(--accent)] border-t border-[var(--border-subtle)] mt-1.5 flex items-center justify-between",
                        span { "Catppuccin Series" }
                        span { class: "text-[9px] font-medium px-1.5 py-0.5 rounded bg-[var(--bg-surface-subtle)] text-[var(--text-muted)]", "Official" }
                    }
                    for opt in CATPPUCCIN_THEMES {
                        {
                            let is_active = current_theme == opt.mode;
                            let mode = opt.mode;
                            rsx! {
                                button {
                                    key: "{opt.label}",
                                    r#type: "button",
                                    class: if is_active { "theme-dropdown-item active" } else { "theme-dropdown-item" },
                                    onclick: move |_| {
                                        on_change_theme.call(mode);
                                        is_open.set(false);
                                    },
                                    div { class: "theme-dropdown-dots",
                                        div {
                                            class: "theme-dropdown-dot",
                                            style: "background-color: {opt.dot_bg}; {opt.border_style}",
                                        }
                                        div {
                                            class: "theme-dropdown-dot",
                                            style: "background-color: {opt.dot_surface}; {opt.border_style}",
                                        }
                                        div {
                                            class: "theme-dropdown-dot",
                                            style: "background-color: {opt.dot_accent};",
                                        }
                                    }
                                    span { class: "flex-1 truncate text-left", "{opt.label}" }
                                    if is_active {
                                        span { class: "text-[var(--accent)] shrink-0 ml-auto",
                                            IconCheck { size: "13" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
