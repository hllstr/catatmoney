use dioxus::prelude::*;

#[component]
pub fn IconWallet(#[props(default = "20")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1" }
            path { d: "M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4" }
        }
    }
}

#[component]
pub fn IconSun(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "4" }
            path { d: "M12 2v2" }
            path { d: "M12 20v2" }
            path { d: "m4.93 4.93 1.41 1.41" }
            path { d: "m17.66 17.66 1.41 1.41" }
            path { d: "M2 12h2" }
            path { d: "M20 12h2" }
            path { d: "m6.34 17.66-1.41 1.41" }
            path { d: "m19.07 4.93-1.41 1.41" }
        }
    }
}

#[component]
pub fn IconMoon(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
        }
    }
}

#[component]
pub fn IconLayoutDashboard(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { width: "7", height: "9", x: "3", y: "3", rx: "1" }
            rect { width: "7", height: "5", x: "14", y: "3", rx: "1" }
            rect { width: "7", height: "9", x: "14", y: "12", rx: "1" }
            rect { width: "7", height: "5", x: "3", y: "16", rx: "1" }
        }
    }
}

#[component]
pub fn IconCalendar(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { width: "18", height: "18", x: "3", y: "4", rx: "2" }
            line { x1: "16", x2: "16", y1: "2", y2: "6" }
            line { x1: "8", x2: "8", y1: "2", y2: "6" }
            line { x1: "3", x2: "21", y1: "10", y2: "10" }
        }
    }
}

#[component]
pub fn IconList(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "8", x2: "21", y1: "6", y2: "6" }
            line { x1: "8", x2: "21", y1: "12", y2: "12" }
            line { x1: "8", x2: "21", y1: "18", y2: "18" }
            line { x1: "3", x2: "3.01", y1: "6", y2: "6" }
            line { x1: "3", x2: "3.01", y1: "12", y2: "12" }
            line { x1: "3", x2: "3.01", y1: "18", y2: "18" }
        }
    }
}

#[component]
pub fn IconArrowUpRight(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M7 17L17 7" }
            path { d: "M7 7h10v10" }
        }
    }
}

#[component]
pub fn IconArrowDownRight(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M7 7l10 10" }
            path { d: "M17 7v10H7" }
        }
    }
}

#[component]
pub fn IconArrowLeftRight(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m16 3 4 4-4 4" }
            path { d: "M20 7H4" }
            path { d: "m8 21-4-4 4-4" }
            path { d: "M4 17h16" }
        }
    }
}

#[component]
pub fn IconPlus(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 5v14" }
            path { d: "M5 12h14" }
        }
    }
}

#[component]
pub fn IconTrash(#[props(default = "15")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M3 6h18" }
            path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
            path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
            line { x1: "10", y1: "11", x2: "10", y2: "17" }
            line { x1: "14", y1: "11", x2: "14", y2: "17" }
        }
    }
}

#[component]
pub fn IconClock(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "10" }
            polyline { points: "12 6 12 12 16 14" }
        }
    }
}

#[component]
pub fn IconPaperclip(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" }
        }
    }
}

#[component]
pub fn IconX(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M18 6 6 18" }
            path { d: "m6 6 12 12" }
        }
    }
}

#[component]
pub fn IconChevronLeft(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m15 18-6-6 6-6" }
        }
    }
}

#[component]
pub fn IconChevronRight(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m9 18 6-6-6-6" }
        }
    }
}

#[component]
pub fn IconReceipt(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" }
            path { d: "M16 8h-8" }
            path { d: "M16 12h-8" }
            path { d: "M12 16h-4" }
        }
    }
}

#[component]
pub fn IconUtensils(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M18 2v6a3 3 0 0 1-3 3 3 3 0 0 1-3-3V2" }
            path { d: "M15 2v10" }
            path { d: "M15 12v10" }
            path { d: "M5 2c0 2.5 1.5 4 4 4v16" }
        }
    }
}

#[component]
pub fn IconCar(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.5 2.8C2 10.9 2 11.2 2 11.5V16c0 .6.4 1 1 1h2" }
            circle { cx: "7", cy: "17", r: "2" }
            path { d: "M9 17h6" }
            circle { cx: "17", cy: "17", r: "2" }
        }
    }
}

#[component]
pub fn IconShoppingBag(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M6 2 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4Z" }
            path { d: "M3 6h18" }
            path { d: "M16 10a4 4 0 0 1-8 0" }
        }
    }
}

#[component]
pub fn IconGamepad(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "6", y1: "12", x2: "10", y2: "12" }
            line { x1: "8", y1: "10", x2: "8", y2: "14" }
            line { x1: "15", y1: "13", x2: "15.01", y2: "13" }
            line { x1: "18", y1: "11", x2: "18.01", y2: "11" }
            rect { width: "20", height: "12", x: "2", y: "6", rx: "6" }
        }
    }
}

#[component]
pub fn IconHeartPulse(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }
            path { d: "M3.22 12H9.5l1.5-3 2 6 1.5-3h6.28" }
        }
    }
}

#[component]
pub fn IconBriefcase(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { width: "20", height: "14", x: "2", y: "7", rx: "2" }
            path { d: "M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" }
        }
    }
}

#[component]
pub fn IconLaptop(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M20 16V7a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v9m16 0H4m16 0 1.28 2.55a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45L4 16" }
        }
    }
}

#[component]
pub fn IconLineChart(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M3 3v18h18" }
            path { d: "m19 9-5 5-4-4-3 3" }
        }
    }
}

#[component]
pub fn IconActivity(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M22 12h-4l-3 9L9 3l-3 9H2" }
        }
    }
}


#[component]
pub fn IconGift(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "3", y: "8", width: "18", height: "4", rx: "1" }
            path { d: "M12 8v13" }
            path { d: "M19 12v7a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-7" }
            path { d: "M7.5 8a2.5 2.5 0 0 1 0-5A4.8 8 0 0 1 12 8a4.8 8 0 0 1 4.5-5 2.5 2.5 0 0 1 0 5" }
        }
    }
}

#[component]
pub fn IconTag(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 2H2v10l9.29 9.29c.94.94 2.48.94 3.42 0l6.58-6.58c.94-.94.94-2.48 0-3.42L12 2Z" }
            path { d: "M7 7h.01" }
        }
    }
}

#[component]
pub fn CategoryIcon(category: String) -> Element {
    match category.as_str() {
        "Makanan & Minuman" => rsx! { IconUtensils {} },
        "Transportasi" => rsx! { IconCar {} },
        "Belanja" => rsx! { IconShoppingBag {} },
        "Tagihan" => rsx! { IconReceipt {} },
        "Hiburan" => rsx! { IconGamepad {} },
        "Kesehatan" => rsx! { IconHeartPulse {} },
        "Gaji" => rsx! { IconBriefcase {} },
        "Freelance" => rsx! { IconLaptop {} },
        "Investasi" => rsx! { IconLineChart {} },
        "Hadiah" => rsx! { IconGift {} },
        _ => rsx! { IconTag {} },
    }
}

#[component]
pub fn IconPieChart(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M21.21 15.89A10 10 0 1 1 8 2.83" }
            path { d: "M22 12A10 10 0 0 0 12 2v10z" }
        }
    }
}

#[component]
pub fn IconArrowRight(#[props(default = "14")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M5 12h14" }
            path { d: "m12 5 7 7-7 7" }
        }
    }
}

#[component]
pub fn IconEdit(#[props(default = "15")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" }
            path { d: "M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" }
        }
    }
}

#[component]
pub fn IconEye(#[props(default = "15")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" }
            circle { cx: "12", cy: "12", r: "3" }
        }
    }
}

#[component]
pub fn IconCreditCard(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { width: "20", height: "14", x: "2", y: "5", rx: "2" }
            line { x1: "2", x2: "22", y1: "10", y2: "10" }
        }
    }
}

#[component]
pub fn IconLandmark(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "3", x2: "21", y1: "22", y2: "22" }
            line { x1: "6", x2: "6", y1: "18", y2: "11" }
            line { x1: "10", x2: "10", y1: "18", y2: "11" }
            line { x1: "14", x2: "14", y1: "18", y2: "11" }
            line { x1: "18", x2: "18", y1: "18", y2: "11" }
            polygon { points: "12 2 20 7 4 7" }
        }
    }
}

#[component]
pub fn IconSmartphone(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { width: "14", height: "20", x: "5", y: "2", rx: "2", ry: "2" }
            path { d: "M12 18h.01" }
        }
    }
}

#[component]
pub fn IconBanknote(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { width: "20", height: "12", x: "2", y: "6", rx: "2" }
            circle { cx: "12", cy: "12", r: "2" }
            path { d: "M6 12h.01M18 12h.01" }
        }
    }
}

#[component]
pub fn IconFolderPlus(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 10v6" }
            path { d: "M9 13h6" }
            path { d: "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" }
        }
    }
}

#[component]
pub fn WalletIcon(wallet_type: crate::model::WalletType, #[props(default = "16")] size: &'static str) -> Element {
    match wallet_type {
        crate::model::WalletType::Bank => rsx! { IconLandmark { size } },
        crate::model::WalletType::EWallet => rsx! { IconSmartphone { size } },
        crate::model::WalletType::Cash => rsx! { IconBanknote { size } },
        crate::model::WalletType::CreditCard => rsx! { IconCreditCard { size } },
        crate::model::WalletType::Other => rsx! { IconWallet { size } },
    }
}

#[component]
pub fn IconSliders(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "4", x2: "4", y1: "21", y2: "14" }
            line { x1: "4", x2: "4", y1: "10", y2: "3" }
            line { x1: "12", x2: "12", y1: "21", y2: "12" }
            line { x1: "12", x2: "12", y1: "8", y2: "3" }
            line { x1: "20", x2: "20", y1: "21", y2: "16" }
            line { x1: "20", x2: "20", y1: "12", y2: "3" }
            line { x1: "1", x2: "7", y1: "14", y2: "14" }
            line { x1: "9", x2: "15", y1: "8", y2: "8" }
            line { x1: "17", x2: "23", y1: "16", y2: "16" }
        }
    }
}

#[component]
pub fn IconCheck(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline { points: "20 6 9 17 4 12" }
        }
    }
}

#[component]
pub fn IconDownload(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
            polyline { points: "7 10 12 15 17 10" }
            line { x1: "12", x2: "12", y1: "15", y2: "3" }
        }
    }
}

#[component]
pub fn IconUpload(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
            polyline { points: "17 8 12 3 7 8" }
            line { x1: "12", x2: "12", y1: "3", y2: "15" }
        }
    }
}

#[component]
pub fn IconAlertTriangle(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" }
            line { x1: "12", x2: "12", y1: "9", y2: "13" }
            line { x1: "12", x2: "12.01", y1: "17", y2: "17" }
        }
    }
}

#[component]
pub fn IconFileText(#[props(default = "16")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
            polyline { points: "14 2 14 8 20 8" }
            line { x1: "16", x2: "8", y1: "13", y2: "13" }
            line { x1: "16", x2: "8", y1: "17", y2: "17" }
            line { x1: "10", x2: "8", y1: "9", y2: "9" }
        }
    }
}

#[component]
pub fn IconUser(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" }
            circle { cx: "12", cy: "7", r: "4" }
        }
    }
}

#[component]
pub fn IconShield(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" }
        }
    }
}

#[component]
pub fn IconSparkles(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" }
            path { d: "M5 3v4" }
            path { d: "M19 17v4" }
            path { d: "M3 5h4" }
            path { d: "M17 19h4" }
        }
    }
}

#[component]
pub fn IconTrendingUp(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline { points: "22 7 13.5 15.5 8.5 10.5 2 17" }
            polyline { points: "16 7 22 7 22 13" }
        }
    }
}

#[component]
pub fn IconTrendingDown(#[props(default = "18")] size: &'static str) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline { points: "22 17 13.5 8.5 8.5 13.5 2 7" }
            polyline { points: "16 17 22 17 22 11" }
        }
    }
}




