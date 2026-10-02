use dioxus::prelude::*;
use crate::components::icons::{
    IconCalendar, IconLayoutDashboard, IconList, IconPlus, IconSliders,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NavTab {
    Dashboard,
    Calendar,
    History,
    Management,
}

#[component]
pub fn BottomNavBar(
    active_tab: NavTab,
    on_select_tab: EventHandler<NavTab>,
    on_open_modal: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "bottom-dock-wrapper",
            nav { class: "bottom-dock",
                // Tab Dashboard
                button {
                    r#type: "button",
                    class: if active_tab == NavTab::Dashboard { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Dashboard),
                    IconLayoutDashboard { size: "18" }
                    span { class: "dock-label", "Dashboard" }
                }

                // Tab Kalender Keuangan
                button {
                    r#type: "button",
                    class: if active_tab == NavTab::Calendar { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Calendar),
                    IconCalendar { size: "18" }
                    span { class: "dock-label", "Kalender" }
                }

                // Tombol Aksi Tambah Transaksi Utama (+) di Tengah
                div { class: "dock-fab-container",
                    button {
                        r#type: "button",
                        class: "dock-fab-btn",
                        title: "Tambah Catatan Uang",
                        onclick: move |_| on_open_modal.call(()),
                        IconPlus { size: "22" }
                    }
                }

                // Tab Riwayat Transaksi
                button {
                    r#type: "button",
                    class: if active_tab == NavTab::History { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::History),
                    IconList { size: "18" }
                    span { class: "dock-label", "Riwayat" }
                }

                // Tab Manajemen / Kelola Kategori & Sumber Dana
                button {
                    r#type: "button",
                    class: if active_tab == NavTab::Management { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Management),
                    IconSliders { size: "18" }
                    span { class: "dock-label", "Kelola" }
                }
            }
        }
    }
}
