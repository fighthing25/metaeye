# Venom Island — Dynamic Island untuk Windows

Kapsul hitam di tengah atas layar. Selalu di atas, transparan, elastis mengikuti kursor, berwajah Venom.
Dibuat 100% Rust + egui/eframe. Ringan, tanpa Electron.

## Fitur v1.0

**Tampilan & Gerak**
- Kapsul hitam tengah atas, selalu di atas, transparan
- Membal elastis jadi busur saat kursor mendekat, ikon fitur + tooltip
- Klik ikon -> radial meregang + kotak pengaturan muncul di tengah
- Scroll kanan = volume, kiri = kecerahan (kecerahan hanya layar laptop)
- Pulau menghilang otomatis setelah 1 detik tanpa kursor (mode redup/mini)
- Tema AMOLED + Liquid Glass, animasi bisa diatur, warna tema, transparansi, sensitivitas kursor, suara retro 8-bit

**Wajah Venom**
- Mata miring + glow, mengikuti kursor, berkedip
- Baterai rendah/kritis/mengisi/penuh + ikon charger
- Berkeringat saat CPU panas, pusing saat file diseret, mengangguk saat musik, berkacamata saat mode fokus, minum kopi saat Awake, indikator kamera/mikrofon, melompat lalu kaget saat timer selesai

**Musik & Suara**
- Kontrol lagu (sebelumnya, jeda, berikutnya), mode musik: pulau memanjang, visualizer kiri, sampul album kanan, lirik per baris dari lrclib.net
- Bisukan speaker/mikrofon, slider volume & kecerahan, pengalih output suara

**Sistem**
- Screenshot penuh/area/jendela/rekam layar, pipet warna HEX/RGB, Awake, kunci, tidur/restart/matikan + konfirmasi + tampilan unik, indikator Caps Lock, kunci jendela selalu di atas, traffic monitor, task manager mini, Wi-Fi, Pomodoro/timer, mode gaming, pelacak unduhan

**Pusat Data**
- Jadwal shalat: pilih kota, semua waktu, hitung mundur, pengingat bunyi Windows (on/off), imsak & berbuka saat Ramadan
- Kalender Masehi + pasaran Jawa + Hijriah (bisa disembunyikan), tanggal bisa diklik, tanggal merah Indonesia 2026 resmi + tahun lain perkiraan, tugas per tanggal
- Sambung kalender Google/Microsoft (placeholder OAuth), clipboard + OTP auto-hapus, rak file (drag ke pulau), sambung HP (file & clipboard lewat browser, PIN, QR)

**Addons & Pengaturan**
- Tombol Addons toggle (shalat, traffic), Teleprompter, pengaturan utama, ekspor/impor .txt, folder pelacak unduhan, dukung kreator (QRIS Trakteer, Ko-fi, Buy Me a Coffee)

## Cara Menjalankan

Windows (disarankan):
```powershell
winget install Rustlang.Rustup
cargo run --release
```

Linux/macOS preview:
```bash
cargo run --release
```

Build EXE:
```powershell
cargo build --release
# target/release/venom-island.exe  (target <10 MB dengan strip+lto)
```

## Push ke GitHub
```bash
git init
git add .
git commit -m "feat: Venom Island v1.0"
git branch -M main
git remote add origin https://github.com/USERNAME-KAMU/venom-island.git
git push -u origin main
```
GitHub Actions di `.github/workflows/build.yml` akan build EXE otomatis tiap push.

## Gestur
| Gestur | Aksi |
|---|---|
| Hover kapsul | Bangun + membal elastis |
| Klik kapsul | Buka radial |
| Scroll sisi kanan kapsul | Volume |
| Scroll sisi kiri kapsul | Kecerahan |
| Drag file ke kapsul | Masuk Rak File + wajah pusing |
| Diam 1 detik tanpa kursor | Pulau meredup/mini |

## File Data
- `settings.json` — pengaturan (otomatis)
- `tasks.json` — tugas per tanggal
- `clipboard.json` — riwayat clipboard

## Sambung HP
Pengaturan > Sambung HP > Aktifkan Server > buka `http://IP-KOMPUTER:8080` di HP (satu Wi-Fi), PIN default `123456`.

## Catatan
- Jadwal shalat: API Aladhan (gratis). Offline -> fallback Jakarta/WIB.
- Lirik: lrclib.net (gratis).
- Tanggal merah 2026: SKB 3 Menteri. Tahun lain perkiraan.
- Pasaran Jawa: referensi 1 Jan 1970 = Kamis Wage.
- Hijriah: konversi tabular Islamic (±1 hari).

MIT License.
