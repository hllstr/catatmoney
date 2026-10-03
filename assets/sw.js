// Service Worker untuk CatatMoney - 100% Full Offline PWA
// Menggunakan strategi Cache-First untuk aset statis/WASM dan Network-First dengan fallback cache untuk navigasi.

const CACHE_VERSION = 'catatmoney-v1.0.0';
const STATIC_CACHE = `${CACHE_VERSION}-static`;
const FONT_CACHE = `${CACHE_VERSION}-fonts`;

// Daftar aset dasar yang wajib ada di precache
const CORE_PRECACHE_URLS = [
  '/catatmoney/',
  '/catatmoney/index.html',
  '/catatmoney/manifest.json',
  '/catatmoney/icon.svg',
  '/catatmoney/style.css',
  '/catatmoney/tailwind.css',
];

// Placeholder yang akan diinjeksi saat proses build dengan daftar seluruh file release (WASM, JS hash, dll.)
const INJECTED_ASSETS = self.__WB_MANIFEST || [];

const ALL_PRECACHE_URLS = Array.from(new Set([...CORE_PRECACHE_URLS, ...INJECTED_ASSETS]));

// Event Install: Simpan semua aset inti ke dalam Cache Storage
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(STATIC_CACHE).then(async (cache) => {
      // Gunakan penanganan toleran agar jika satu URL opsional gagal, worker tetap aktif
      await Promise.allSettled(
        ALL_PRECACHE_URLS.map(async (url) => {
          try {
            const response = await fetch(url, { cache: 'no-cache' });
            if (response.ok) {
              await cache.put(url, response);
            }
          } catch (err) {
            console.warn('[SW] Gagal precache URL:', url, err);
          }
        })
      );
    }).then(() => self.skipWaiting())
  );
});

// Event Activate: Bersihkan cache versi lama dan klaim kontrol client secara instan
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) => {
      return Promise.all(
        keys.map((key) => {
          if (key !== STATIC_CACHE && key !== FONT_CACHE) {
            console.log('[SW] Menghapus cache usang:', key);
            return caches.delete(key);
          }
        })
      );
    }).then(() => self.clients.claim())
  );
});

// Event Fetch: Intersepsi permintaan jaringan untuk offline-first
self.addEventListener('fetch', (event) => {
  const request = event.request;
  const url = new URL(request.url);

  // Hanya tangani metode GET pada protokol http/https
  if (request.method !== 'GET' || !url.protocol.startsWith('http')) {
    return;
  }

  // 1. Google Fonts & Web Fonts (fonts.googleapis.com, fonts.gstatic.com)
  if (url.hostname === 'fonts.googleapis.com' || url.hostname === 'fonts.gstatic.com') {
    event.respondWith(
      caches.open(FONT_CACHE).then(async (cache) => {
        const cachedResponse = await cache.match(request);
        if (cachedResponse) {
          // Kembalikan dari cache segera, perbarui di background jika online
          fetch(request).then((networkResponse) => {
            if (networkResponse && networkResponse.ok) {
              cache.put(request, networkResponse.clone());
            }
          }).catch(() => {});
          return cachedResponse;
        }

        try {
          const networkResponse = await fetch(request);
          if (networkResponse && networkResponse.ok) {
            cache.put(request, networkResponse.clone());
          }
          return networkResponse;
        } catch {
          return new Response('', { status: 503, statusText: 'Font Service Unavailable Offline' });
        }
      })
    );
    return;
  }

  // 2. Permintaan Navigasi HTML (User membuka halaman / reload)
  if (request.mode === 'navigate' || request.destination === 'document') {
    event.respondWith(
      fetch(request)
        .then(async (networkResponse) => {
          if (networkResponse && networkResponse.ok) {
            const cache = await caches.open(STATIC_CACHE);
            cache.put('/catatmoney/index.html', networkResponse.clone());
            cache.put(request, networkResponse.clone());
          }
          return networkResponse;
        })
        .catch(async () => {
          // Mode OFFLINE: Kembalikan index.html dari cache
          const cache = await caches.open(STATIC_CACHE);
          const cachedIndex = await cache.match('/catatmoney/index.html') || await cache.match('/catatmoney/');
          if (cachedIndex) {
            return cachedIndex;
          }
          return new Response('Aplikasi CatatMoney Offline. Silakan buka aplikasi saat terhubung kembali.', {
            headers: { 'Content-Type': 'text/plain; charset=utf-8' }
          });
        })
    );
    return;
  }

  // 3. Aset Statis Lokal (/catatmoney/*, .wasm, .js, .css, .svg, .json)
  if (url.origin === self.location.origin) {
    event.respondWith(
      caches.open(STATIC_CACHE).then(async (cache) => {
        // Coba cocokan di cache (Cache-First)
        const cachedResponse = await cache.match(request, { ignoreSearch: true });
        if (cachedResponse) {
          return cachedResponse;
        }

        // Jika belum ada di cache, ambil dari network dan simpan ke cache
        try {
          const networkResponse = await fetch(request);
          if (networkResponse && networkResponse.ok) {
            cache.put(request, networkResponse.clone());
          }
          return networkResponse;
        } catch (fetchError) {
          // Coba fallback jika path berakhir dengan slash atau index
          if (url.pathname === '/catatmoney/' || url.pathname === '/catatmoney') {
            const fallbackIndex = await cache.match('/catatmoney/index.html');
            if (fallbackIndex) return fallbackIndex;
          }
          throw fetchError;
        }
      })
    );
  }
});
