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

export function playTactileTap() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Layer 1: Ketukan padat "tuk" (pitch-dropped triangle wave)
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = 'triangle';
        osc.frequency.setValueAtTime(320, now);
        osc.frequency.exponentialRampToValueAtTime(75, now + 0.035);
        
        gain.gain.setValueAtTime(0.001, now);
        gain.gain.linearRampToValueAtTime(0.35, now + 0.002);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.038);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        osc.start(now);
        osc.stop(now + 0.04);

        // Layer 2: Transien klik mekanikal kontak switch
        const clickOsc = ctx.createOscillator();
        const clickGain = ctx.createGain();
        clickOsc.type = 'sine';
        clickOsc.frequency.setValueAtTime(2400, now);
        clickOsc.frequency.exponentialRampToValueAtTime(800, now + 0.006);
        
        clickGain.gain.setValueAtTime(0.001, now);
        clickGain.gain.linearRampToValueAtTime(0.12, now + 0.001);
        clickGain.gain.exponentialRampToValueAtTime(0.0001, now + 0.008);
        
        clickOsc.connect(clickGain);
        clickGain.connect(ctx.destination);
        clickOsc.start(now);
        clickOsc.stop(now + 0.01);
    } catch (e) {}
}

export function playCoinChime() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // 1. Benturan koin logam awal ("ck-")
        const osc1 = ctx.createOscillator();
        const gain1 = ctx.createGain();
        osc1.type = 'sine';
        osc1.frequency.setValueAtTime(3520, now);
        osc1.frequency.exponentialRampToValueAtTime(2600, now + 0.05);
        gain1.gain.setValueAtTime(0.001, now);
        gain1.gain.linearRampToValueAtTime(0.26, now + 0.002);
        gain1.gain.exponentialRampToValueAtTime(0.0001, now + 0.06);
        osc1.connect(gain1);
        gain1.connect(ctx.destination);
        osc1.start(now);
        osc1.stop(now + 0.06);

        // 2. Pantulan denting koin emas berkilau ("-ringgg")
        const t2 = now + 0.045;
        const osc2 = ctx.createOscillator();
        const gain2 = ctx.createGain();
        osc2.type = 'sine';
        osc2.frequency.setValueAtTime(4186, t2);
        osc2.frequency.exponentialRampToValueAtTime(4400, t2 + 0.35);
        gain2.gain.setValueAtTime(0.001, t2);
        gain2.gain.linearRampToValueAtTime(0.30, t2 + 0.003);
        gain2.gain.exponentialRampToValueAtTime(0.0001, t2 + 0.38);
        osc2.connect(gain2);
        gain2.connect(ctx.destination);
        osc2.start(t2);
        osc2.stop(t2 + 0.38);

        // 3. Harmonik overtone kemilau tinggi
        const osc3 = ctx.createOscillator();
        const gain3 = ctx.createGain();
        osc3.type = 'sine';
        osc3.frequency.setValueAtTime(6272, t2);
        osc3.frequency.exponentialRampToValueAtTime(6600, t2 + 0.25);
        gain3.gain.setValueAtTime(0.001, t2);
        gain3.gain.linearRampToValueAtTime(0.18, t2 + 0.003);
        gain3.gain.exponentialRampToValueAtTime(0.0001, t2 + 0.28);
        osc3.connect(gain3);
        gain3.connect(ctx.destination);
        osc3.start(t2);
        osc3.stop(t2 + 0.28);
    } catch (e) {}
}

export function playCashRustle() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // 1. Noise buffer tekstur kertas uang
        const duration = 0.22;
        const bufferSize = Math.floor(ctx.sampleRate * duration);
        const noiseBuffer = ctx.createBuffer(1, bufferSize, ctx.sampleRate);
        const output = noiseBuffer.getChannelData(0);
        for (let i = 0; i < bufferSize; i++) {
            output[i] = Math.random() * 2 - 1;
        }
        
        const whiteNoise = ctx.createBufferSource();
        whiteNoise.buffer = noiseBuffer;
        
        // 2. Bandpass sweep friksi lembaran uang ("sresss")
        const filter = ctx.createBiquadFilter();
        filter.type = 'bandpass';
        filter.Q.setValueAtTime(2.2, now);
        filter.frequency.setValueAtTime(1500, now);
        filter.frequency.exponentialRampToValueAtTime(3400, now + 0.06);
        filter.frequency.exponentialRampToValueAtTime(1900, now + 0.20);
        
        const gain = ctx.createGain();
        gain.gain.setValueAtTime(0.001, now);
        gain.gain.linearRampToValueAtTime(0.28, now + 0.018);
        gain.gain.setValueAtTime(0.25, now + 0.08);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + duration);
        
        whiteNoise.connect(filter);
        filter.connect(gain);
        gain.connect(ctx.destination);
        
        // 3. Jentikan jari awal saat mengeluarkan uang
        const flick = ctx.createOscillator();
        const flickGain = ctx.createGain();
        flick.type = 'sine';
        flick.frequency.setValueAtTime(220, now);
        flick.frequency.exponentialRampToValueAtTime(80, now + 0.025);
        flickGain.gain.setValueAtTime(0.001, now);
        flickGain.gain.linearRampToValueAtTime(0.12, now + 0.002);
        flickGain.gain.exponentialRampToValueAtTime(0.0001, now + 0.03);
        flick.connect(flickGain);
        flickGain.connect(ctx.destination);
        flick.start(now);
        flick.stop(now + 0.03);
        
        whiteNoise.start(now);
        whiteNoise.stop(now + duration);
    } catch (e) {}
}

export function playTransferWoosh() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Akor futuristik transfer dana naik
        const osc1 = ctx.createOscillator();
        const gain1 = ctx.createGain();
        osc1.type = 'sine';
        osc1.frequency.setValueAtTime(523.25, now);
        osc1.frequency.exponentialRampToValueAtTime(783.99, now + 0.16);
        gain1.gain.setValueAtTime(0.001, now);
        gain1.gain.linearRampToValueAtTime(0.22, now + 0.003);
        gain1.gain.exponentialRampToValueAtTime(0.0001, now + 0.22);
        osc1.connect(gain1);
        gain1.connect(ctx.destination);
        osc1.start(now);
        osc1.stop(now + 0.22);
        
        const osc2 = ctx.createOscillator();
        const gain2 = ctx.createGain();
        osc2.type = 'triangle';
        osc2.frequency.setValueAtTime(659.25, now + 0.04);
        osc2.frequency.exponentialRampToValueAtTime(1046.50, now + 0.18);
        gain2.gain.setValueAtTime(0.001, now + 0.04);
        gain2.gain.linearRampToValueAtTime(0.18, now + 0.043);
        gain2.gain.exponentialRampToValueAtTime(0.0001, now + 0.24);
        osc2.connect(gain2);
        gain2.connect(ctx.destination);
        osc2.start(now + 0.04);
        osc2.stop(now + 0.24);
    } catch (e) {}
}

export function playSuccessChime() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Dua nada naik lembut (D5 ke A5)
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(587.33, now);
        osc.frequency.exponentialRampToValueAtTime(880, now + 0.08);
        
        gain.gain.setValueAtTime(0.001, now);
        gain.gain.linearRampToValueAtTime(0.22, now + 0.002);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.20);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        
        osc.start(now);
        osc.stop(now + 0.20);
    } catch (e) {}
}

export function playDeleteThud() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Hantaman rendah redam mantap
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(180, now);
        osc.frequency.exponentialRampToValueAtTime(55, now + 0.09);
        
        gain.gain.setValueAtTime(0.001, now);
        gain.gain.linearRampToValueAtTime(0.28, now + 0.002);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.12);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        
        osc.start(now);
        osc.stop(now + 0.12);
    } catch (e) {}
}

export function playInfoPop() {
    try {
        const ctx = getAudioContext();
        if (!ctx) return;
        const now = ctx.currentTime;
        
        // Nada pop tactile pendek "tut"
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = 'triangle';
        osc.frequency.setValueAtTime(520, now);
        osc.frequency.exponentialRampToValueAtTime(260, now + 0.045);
        
        gain.gain.setValueAtTime(0.001, now);
        gain.gain.linearRampToValueAtTime(0.22, now + 0.002);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.05);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        
        osc.start(now);
        osc.stop(now + 0.05);
    } catch (e) {}
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
        osc.frequency.setValueAtTime(349.23, now);
        
        gain.gain.setValueAtTime(0.001, now);
        gain.gain.linearRampToValueAtTime(0.20, now + 0.002);
        gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.16);
        
        osc.connect(gain);
        gain.connect(ctx.destination);
        
        osc.start(now);
        osc.stop(now + 0.16);
    } catch (e) {}
}

export function initAudio() {
    if (typeof window === 'undefined') return;
    if (window.__catatmoney_audio_initialized) return;
    window.__catatmoney_audio_initialized = true;

    let lastTapTime = 0;
    
    // Listener interaksi pointerdown untuk feedback instan 0ms tactile tap ("tuk")
    window.addEventListener('pointerdown', (e) => {
        try {
            if (localStorage.getItem('catatmoney_sound_fx_v1') === 'false') return;
            const target = e.target;
            if (!target || !target.closest) return;
            
            // Hindari trigger saat fokus ke field input teks atau textarea
            if (target.closest('input:not([type="button"]):not([type="submit"]):not([type="checkbox"]):not([type="radio"]), textarea, [contenteditable="true"]')) {
                return;
            }
            
            // Cek elemen interaktif
            const interactive = target.closest('button, [role="button"], a, input[type="checkbox"], input[type="radio"], select, .filter-pill, .nav-item, .theme-card, .clickable, .tab-btn, .transaction-row, summary, [onclick], .cursor-pointer');
            if (interactive) {
                if (interactive.disabled || interactive.getAttribute('aria-disabled') === 'true') return;
                const now = performance.now();
                if (now - lastTapTime < 45) return;
                lastTapTime = now;
                playTactileTap();
            }
        } catch (err) {}
    }, { passive: true });

    // Listener interaksi keyboard (Enter & Space pada elemen interaktif yang sedang fokus)
    window.addEventListener('keydown', (e) => {
        try {
            if (localStorage.getItem('catatmoney_sound_fx_v1') === 'false') return;
            if (e.key === 'Enter' || e.key === ' ') {
                const active = document.activeElement;
                if (active && (active.tagName === 'BUTTON' || active.getAttribute('role') === 'button' || active.classList.contains('filter-pill') || active.classList.contains('nav-item') || active.classList.contains('tab-btn'))) {
                    const now = performance.now();
                    if (now - lastTapTime < 45) return;
                    lastTapTime = now;
                    playTactileTap();
                }
            }
        } catch (err) {}
    }, { passive: true });
}

// Inisialisasi otomatis jika lingkungan window sudah tersedia
if (typeof window !== 'undefined') {
    initAudio();
}
"#)]
#[allow(non_snake_case)]
extern "C" {
    fn initAudio();
    fn playTactileTap();
    fn playCoinChime();
    fn playCashRustle();
    fn playTransferWoosh();
    fn playSuccessChime();
    fn playDeleteThud();
    fn playInfoPop();
    fn playWarningBeep();
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn initAudio() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn playTactileTap() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn playCoinChime() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn playCashRustle() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn playTransferWoosh() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn playSuccessChime() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn playDeleteThud() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn playInfoPop() {}
#[cfg(not(target_arch = "wasm32"))]
#[allow(non_snake_case, dead_code)]
fn playWarningBeep() {}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundEffect {
    Tap,
    Coin,
    Cash,
    Transfer,
    Success,
    Delete,
    Info,
    Warning,
}

/// Inisialisasi pendengar klik interaktif global
pub fn init_audio() {
    #[cfg(target_arch = "wasm32")]
    {
        initAudio();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        initAudio();
    }
}

/// Memutar efek suara mikro secara procedural jika diizinkan pengguna
pub fn play_sound(effect: SoundEffect, is_enabled: bool) {
    if !is_enabled {
        return;
    }
    match effect {
        SoundEffect::Tap => playTactileTap(),
        SoundEffect::Coin => playCoinChime(),
        SoundEffect::Cash => playCashRustle(),
        SoundEffect::Transfer => playTransferWoosh(),
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
