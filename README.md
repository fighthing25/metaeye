# Venom Island - FIXED v1.0.1

Perbaikan 7 error:
- toggle_panel(&Panel) borrow, bukan ownership (clippy + E0382/E0502)
- Slider timer i64: 10i64..=3600i64 (E0308)
- sysinfo: global_cpu_usage() -> global_cpu_info().cpu_usage() (E0599)
- Prayer countdown tanpa and_local_timezone (E0277)
- Radial & panel: clone Panel sebelum match, kumpulkan klik dulu baru mutasi (E0502/E0382)
- Clipboard/FileShelf/Calendar: to_del/to_copy pattern, tanpa break di closure (E0382)
- ureq: into_string + serde_json::from_str, bukan into_json (E0277)
- draw_island: allocate_at_least, bukan allocate_ui_at_rect

Jalankan:
cargo run --release

Build:
cargo build --release
