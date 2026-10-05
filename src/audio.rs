// Audio Synthesizer Engine via Web Audio API (Zero-Asset procedural audio cues)
// Mematuhi GEMINI.md: Zero Emoji, Obsidian Dark Minimalist design principles

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
let audioCtx = null;

function getAudioContext() {
    if (!audioCtx) {
        const AudioContextClass = window.AudioContext || window.webkitAudioContext;
        if (AudioContextClass) {
            audioCtx = new AudioContextClass();
        }
    }
    if (audioCtx && audioCtx.state === 'suspended') {
        audioCtx.resume();
    }
    return audioCtx;
}

export function playSuccessChime() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Dua nada naik lembut (C5 ke G5)
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(523.25, now);
        osc.frequency.exponentialRampToValueAtTime(783.99, now + 0.08);
        
        gain.gain.setValueAtTime(0.08, now);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.16);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        
        osc.start(now);
        osc.stop(now + 0.16);
    } catch (e) {
        // Fallback hening jika audio dilarang browser
    }
}

export function playDeleteThud() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Nada rendah redam pendek
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(220, now);
        osc.frequency.exponentialRampToValueAtTime(110, now + 0.09);
        
        gain.gain.setValueAtTime(0.10, now);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.11);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        
        osc.start(now);
        osc.stop(now + 0.11);
    } catch (e) {
        // Fallback hening
    }
}

export function playInfoPop() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Nada pop lembut pendek
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(659.25, now);
        osc.frequency.exponentialRampToValueAtTime(880, now + 0.05);
        
        gain.gain.setValueAtTime(0.06, now);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.08);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        
        osc.start(now);
        osc.stop(now + 0.08);
    } catch (e) {
        // Fallback hening
    }
}

export function playWarningBeep() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Nada peringatan redam
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = 'triangle';
        osc.frequency.setValueAtTime(329.63, now);
        
        gain.gain.setValueAtTime(0.07, now);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.15);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        
        osc.start(now);
        osc.stop(now + 0.15);
    } catch (e) {
        // Fallback hening
    }
}
"#)]
#[allow(non_snake_case)]
extern "C" {
    fn playSuccessChime();
    fn playDeleteThud();
    fn playInfoPop();
    fn playWarningBeep();
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case)]
fn playSuccessChime() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case)]
fn playDeleteThud() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case)]
fn playInfoPop() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case)]
fn playWarningBeep() {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundEffect {
    Success,
    Delete,
    Info,
    Warning,
}

/// Memutar efek suara mikro secara procedural jika diizinkan pengguna
pub fn play_sound(effect: SoundEffect, is_enabled: bool) {
    if !is_enabled {
        return;
    }
    match effect {
        SoundEffect::Success => playSuccessChime(),
        SoundEffect::Delete => playDeleteThud(),
        SoundEffect::Info => playInfoPop(),
        SoundEffect::Warning => playWarningBeep(),
    }
}

#[allow(dead_code)]
pub const SOUND_FX_KEY: &str = "catatmoney_sound_fx_v1";

/// Memuat preferensi suara dari LocalStorage (default aktif: true)
pub fn load_sound_enabled() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(val)) = storage.get_item(SOUND_FX_KEY) {
                    return val != "false";
                }
            }
        }
    }
    true
}

/// Menyimpan preferensi suara ke LocalStorage
pub fn save_sound_enabled(_enabled: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(SOUND_FX_KEY, if _enabled { "true" } else { "false" });
            }
        }
    }
}

/// Helper timer tidur async di WebAssembly
#[cfg(target_arch = "wasm32")]
pub async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(w) = web_sys::window() {
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn sleep_ms(_ms: i32) {}
