const fs = require('fs');
const path = require('path');

const publicDir = process.env.PUBLIC_DIR 
  ? path.resolve(process.env.PUBLIC_DIR)
  : path.resolve(__dirname, '../target/dx/catatmoney/release/web/public');
const swTemplatePath = path.resolve(__dirname, '../assets/sw.js');
const swDestPath = path.join(publicDir, 'sw.js');

if (!fs.existsSync(publicDir)) {
  console.error('[generate-sw] Public directory does not exist:', publicDir);
  process.exit(1);
}

// Rekursif membaca seluruh berkas dalam direktori publik
function getAllFiles(dir, baseDir = dir) {
  let files = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files = files.concat(getAllFiles(fullPath, baseDir));
    } else {
      const relPath = path.relative(baseDir, fullPath).replace(/\\/g, '/');
      files.push(relPath);
    }
  }
  return files;
}

const allFiles = getAllFiles(publicDir);

// Saring berkas yang harus di-precache
const filtered = allFiles.filter(file => {
  if (file === 'sw.js' || file.endsWith('/sw.js') || file === '404.html') return false;
  if (file.endsWith('.map')) return false;
  return true;
});

// Format dengan base path /catatmoney/
const precacheUrls = filtered.map(file => `/catatmoney/${file}`);

// Buat cache version yang unik berdasarkan timestamp
const cacheVersion = `catatmoney-v${Date.now()}`;

console.log(`[generate-sw] Ditemukan ${precacheUrls.length} berkas aset untuk di-precache:`);
precacheUrls.forEach(url => console.log('  -', url));

let swContent = fs.readFileSync(swTemplatePath, 'utf8');

// Ganti versi cache dan array INJECTED_ASSETS
swContent = swContent.replace(
  /const CACHE_VERSION = '[^']+';/,
  `const CACHE_VERSION = '${cacheVersion}';`
);

swContent = swContent.replace(
  /const INJECTED_ASSETS = self\.__WB_MANIFEST \|\| \[\];/,
  `const INJECTED_ASSETS = ${JSON.stringify(precacheUrls, null, 2)};`
);

// Tulis ke sw.js di folder publik target
fs.writeFileSync(swDestPath, swContent, 'utf8');
console.log(`[generate-sw] Berhasil men-generate ${swDestPath} (Versi: ${cacheVersion})`);
