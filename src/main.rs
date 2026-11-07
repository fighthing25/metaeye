// Venom Island - FIXED v1.0.1 - semua 7 error diperbaiki
// Perubahan: toggle_panel(&Panel) borrow, Slider i64, global_cpu_info(), countdown tanpa timezone, tanpa allocate_ui_at_rect, tanpa break di closure
#![allow(dead_code)]
#![allow(unused_variables)]
use chrono::{Datelike, Local, NaiveDate, Timelike};
use egui::{Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub theme_color: [u8; 4],
    pub transparency: f32,
    pub cursor_sensitivity: f32,
    pub animations: bool,
    pub retro_sound: bool,
    pub show_jawa: bool,
    pub show_hijri: bool,
    pub prayer_notif: bool,
    pub start_with_windows: bool,
    pub city: String,
    pub addons_prayer: bool,
    pub addons_traffic: bool,
    pub gaming_mode: bool,
    pub always_on_top: bool,
    pub download_folder: String,
    pub phone_pin: String,
    pub auto_hide_seconds: f32,
    pub teleprompter_speed: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Self { theme_color: [168,85,247,255], transparency: 0.92, cursor_sensitivity: 1.0, animations: true, retro_sound: true, show_jawa: true, show_hijri: true, prayer_notif: true, start_with_windows: false, city: "Semarang".to_string(), addons_prayer: true, addons_traffic: true, gaming_mode: false, always_on_top: true, download_folder: "Downloads".to_string(), phone_pin: "123456".to_string(), auto_hide_seconds: 1.0, teleprompter_speed: 1.0 }
    }
}
impl Settings {
    pub fn load() -> Self {
        if let Ok(s) = std::fs::read_to_string("settings.json") {
            if let Ok(v) = serde_json::from_str(&s) { return v; }
        }
        Self::default()
    }
    pub fn save(&self) { let _ = std::fs::write("settings.json", serde_json::to_string_pretty(self).unwrap_or_default()); }
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PrayerTimes { pub imsak: String, pub subuh: String, pub terbit: String, pub dhuha: String, pub dzuhur: String, pub ashar: String, pub maghrib: String, pub isya: String, pub hijri_date: String, pub is_ramadan: bool }
impl PrayerTimes {
    pub fn fallback() -> Self { Self { imsak:"04:15".into(), subuh:"04:25".into(), terbit:"05:45".into(), dhuha:"06:10".into(), dzuhur:"11:45".into(), ashar:"15:05".into(), maghrib:"17:50".into(), isya:"19:00".into(), hijri_date:"-".into(), is_ramadan:false } }
    pub fn all(&self) -> Vec<(&'static str, String)> { vec![("Imsak",self.imsak.clone()),("Subuh",self.subuh.clone()),("Terbit",self.terbit.clone()),("Dhuha",self.dhuha.clone()),("Dzuhur",self.dzuhur.clone()),("Ashar",self.ashar.clone()),("Maghrib / Berbuka",self.maghrib.clone()),("Isya",self.isya.clone())] }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Panel { None, Radial, Settings, Music, Screenshot, ColorPicker, Power, System, Calendar, Prayer, Clipboard, FileShelf, Phone, Teleprompter, Pomodoro, TaskManager, Traffic, Support, Addons }
pub fn pasaran_jawa(date: NaiveDate) -> &'static str {
    let epoch = NaiveDate::from_ymd_opt(1970,1,1).unwrap();
    let diff = (date - epoch).num_days();
    let idx = ((diff % 5) + 5) % 5;
    match idx { 0=>"Wage", 1=>"Kliwon", 2=>"Legi", 3=>"Pahing", _=>"Pon" }
}
pub fn hijri_from_gregorian(date: NaiveDate) -> (i32, String, u32) {
    let jd = date.num_days_from_ce() + 1721426;
    let mut l = jd - 1948440 + 10632;
    let n = (l - 1) / 10631;
    l = l - 10631 * n + 354;
    let j = ((10985 - l) / 5316) * ((50 * l) / 17719) + (l / 5670) * ((43 * l) / 15238);
    l = l - ((30 - j) / 15) * ((17719 * j) / 50) - ((j) / 16) * ((15238 * l) / 43) + 29;
    let month = (24 * l) / 709;
    let day = l - (709 * month) / 24;
    let year = 30 * n + j - 30;
    let names = ["Muharram","Safar","Rabiul Awal","Rabiul Akhir","Jumadil Awal","Jumadil Akhir","Rajab","Sya'ban","Ramadan","Syawal","Dzulqa'dah","Dzulhijjah"];
    let mname = if month>=1 && month<=12 { names[(month-1) as usize] } else { "Muharram" };
    (year, mname.to_string(), day as u32)
}
pub fn is_holiday_2026(date: NaiveDate) -> Option<&'static str> {
    if date.year()!=2026 { return None; }
    let m=date.month(); let d=date.day();
    let list: &[(u32,u32,&str)] = &[(1,1,"Tahun Baru Masehi"),(1,16,"Isra Mi'raj Nabi Muhammad SAW"),(2,17,"Tahun Baru Imlek 2577"),(3,19,"Hari Suci Nyepi"),(3,21,"Idul Fitri 1447 H"),(3,22,"Idul Fitri 1447 H"),(4,3,"Wafat Yesus Kristus"),(4,5,"Hari Raya Paskah"),(5,1,"Hari Buruh"),(5,14,"Hari Raya Waisak"),(5,29,"Kenaikan Yesus Kristus"),(6,1,"Hari Lahir Pancasila"),(6,6,"Idul Adha 1447 H"),(6,27,"Tahun Baru Islam 1448 H"),(8,17,"Hari Kemerdekaan RI"),(9,5,"Maulid Nabi Muhammad SAW"),(12,24,"Cuti Bersama Natal"),(12,25,"Hari Raya Natal")];
    for (mm,dd,name) in list { if *mm==m && *dd==d { return Some(*name); } }
    None
}
pub fn is_red_date(date: NaiveDate) -> bool {
    if date.weekday().num_days_from_sunday()==0 { return true; }
    is_holiday_2026(date).is_some()
}
pub fn is_otp(text: &str) -> bool {
    let lower=text.to_lowercase();
    let has=lower.contains("otp")||lower.contains("kode")||lower.contains("verifikasi")||lower.contains("verification");
    let digits: String=text.chars().filter(|c| c.is_ascii_digit()).collect();
    has && digits.len()>=4 && digits.len()<=8
}
pub fn run_powershell(cmd: &str) {
    #[cfg(target_os="windows")] { let _ = std::process::Command::new("powershell").args(["-NoProfile","-Command",cmd]).spawn(); }
    #[cfg(not(target_os="windows"))] { println!("[PowerShell simulasi] {}", cmd); }
}
pub fn retro_beep(freq: u32, ms: u64) {
    #[cfg(target_os="windows")] { run_powershell(&format!("[console]::beep({},{})", freq, ms)); }
    #[cfg(not(target_os="windows"))] { println!("retro beep {}Hz {}ms", freq, ms); }
}
pub fn set_volume_delta(delta: i32) {
    #[cfg(target_os="windows")] { let key=if delta>0 {175} else {174}; for _ in 0..delta.abs().min(10) { run_powershell(&format!("(New-Object -ComObject WScript.Shell).SendKeys([char]{})", key)); } }
    #[cfg(not(target_os="windows"))] { println!("volume delta {}", delta); }
}
pub fn set_brightness(level: i32) { let lvl=level.clamp(5,100); run_powershell(&format!("(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1,{})", lvl)); }
pub fn open_sound_settings() { run_powershell("Start-Process ms-settings:sound"); }
pub fn take_screenshot_full() {
    #[cfg(target_os="windows")] { run_powershell("$p=\"$env:USERPROFILE\\Pictures\\VenomIsland_\"+(Get-Date -Format yyyyMMdd_HHmmss)+\".png\"; Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('{PRTSC}')"); }
    #[cfg(not(target_os="windows"))] { println!("screenshot full (simulasi)"); }
}
pub fn lock_pc() { #[cfg(target_os="windows")] { let _ = std::process::Command::new("rundll32.exe").args(["user32.dll,LockWorkStation"]).spawn(); } }
pub fn sleep_pc() { #[cfg(target_os="windows")] { let _ = std::process::Command::new("rundll32.exe").args(["powrprof.dll,SetSuspendState","0,1,0"]).spawn(); } }
pub fn shutdown_pc() { #[cfg(target_os="windows")] { let _ = std::process::Command::new("shutdown").args(["/s","/t","0"]).spawn(); } }
pub fn restart_pc() { #[cfg(target_os="windows")] { let _ = std::process::Command::new("shutdown").args(["/r","/t","0"]).spawn(); } }
pub fn get_battery() -> (u8, bool) {
    #[cfg(target_os="windows")] {
        if let Ok(out)=std::process::Command::new("powershell").args(["-NoProfile","-Command","(Get-CimInstance Win32_Battery | Select-Object -First 1 -ExpandProperty EstimatedChargeRemaining)"]).output() {
            if let Ok(s)=String::from_utf8(out.stdout) { if let Ok(v)=s.trim().parse::<u8>() { return (v, is_charging_win()); } }
        } return (87,false);
    }
    #[cfg(not(target_os="windows"))] { (87,false) }
}
#[cfg(target_os="windows")]
fn is_charging_win() -> bool {
    if let Ok(out)=std::process::Command::new("powershell").args(["-NoProfile","-Command","(Get-CimInstance Win32_Battery | Select-Object -First 1 -ExpandProperty BatteryStatus)"]).output() {
        if let Ok(s)=String::from_utf8(out.stdout) { return s.trim()=="2"; }
    } false
}
pub fn get_wifi_ssid() -> String {
    #[cfg(target_os="windows")] {
        if let Ok(out)=std::process::Command::new("netsh").args(["wlan","show","interfaces"]).output() {
            let s=String::from_utf8_lossy(&out.stdout);
            for line in s.lines() { if line.contains("SSID") && !line.contains("BSSID") { if let Some(v)=line.split(':').nth(1) { return v.trim().to_string(); } } }
        } return "Wi-Fi".to_string();
    }
    #[cfg(not(target_os="windows"))] { "Wi-Fi (simulasi)".to_string() }
}
pub fn caps_lock_on() -> bool {
    #[cfg(target_os="windows")] {
        if let Ok(out)=std::process::Command::new("powershell").args(["-NoProfile","-Command","[System.Windows.Forms.Control]::IsKeyLocked('CapsLock')"]).output() {
            return String::from_utf8_lossy(&out.stdout).trim().eq_ignore_ascii_case("True");
        } false
    }
    #[cfg(not(target_os="windows"))] { false }
}
pub fn parse_hm(s: &str) -> Option<(u32,u32)> { let mut it=s.split(':'); Some((it.next()?.trim().parse().ok()?, it.next()?.trim().parse().ok()?)) }
pub fn fetch_prayer_times(city: &str) -> PrayerTimes {
    let url=format!("https://api.aladhan.com/v1/timingsByCity?city={}&country=Indonesia&method=20", city.replace(' ', "%20"));
    if let Ok(resp)=ureq::get(&url).timeout(Duration::from_secs(6)).call() {
        if let Ok(body)=resp.into_string() {
            if let Ok(json)=serde_json::from_str::<serde_json::Value>(&body) {
                let t=&json["data"]["timings"];
                let hijri=json["data"]["date"]["hijri"]["month"]["en"].as_str().unwrap_or("").to_string();
                let is_ram=hijri.eq_ignore_ascii_case("Ramadan")||hijri.eq_ignore_ascii_case("Ramadhan");
                let get=|k: &str| -> String { t[k].as_str().unwrap_or("").chars().take(5).collect() };
                return PrayerTimes{ imsak:get("Imsak"), subuh:get("Fajr"), terbit:get("Sunrise"), dhuha:get("Dhuha"), dzuhur:get("Dhuhr"), ashar:get("Asr"), maghrib:get("Maghrib"), isya:get("Isha"), hijri_date:format!("{} {}", json["data"]["date"]["hijri"]["day"].as_str().unwrap_or(""), hijri), is_ramadan:is_ram };
            }
        }
    }
    PrayerTimes::fallback()
}
pub fn fetch_lyrics(artist: &str, track: &str) -> Vec<String> {
    let url=format!("https://lrclib.net/api/get?artist_name={}&track_name={}", artist.replace(' ', "%20"), track.replace(' ', "%20"));
    if let Ok(resp)=ureq::get(&url).timeout(Duration::from_secs(6)).call() {
        if let Ok(body)=resp.into_string() {
            if let Ok(json)=serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(synced)=json["syncedLyrics"].as_str() {
                    let mut out=Vec::new();
                    for line in synced.lines() { if let Some(idx)=line.find(']') { out.push(line[idx+1..].trim().to_string()); } }
                    if !out.is_empty() { return out; }
                }
                if let Some(plain)=json["plainLyrics"].as_str() { return plain.lines().map(|s| s.to_string()).collect(); }
            }
        }
    }
    vec!["Lirik belum tersedia - isi judul di panel Musik".to_string(), "lrclib.net akan mengambil lirik per baris otomatis.".to_string()]
}
pub fn qr_ascii(text: &str) -> String {
    if let Ok(code)=qrcode::QrCode::new(text.as_bytes()) { return code.render::<char>().quiet_zone(false).module_dimensions(1,1).build(); }
    format!("QR untuk: {}", text)
}
pub fn start_phone_server(pin: String, shared_clip: Arc<Mutex<Vec<String>>>) {
    std::thread::spawn(move || {
        let listener=match TcpListener::bind("0.0.0.0:8080") { Ok(l)=>l, Err(_)=>return };
        for stream in listener.incoming() {
            if let Ok(mut s)=stream {
                let mut buf=[0u8; 8192];
                let _=s.read(&mut buf);
                let req=String::from_utf8_lossy(&buf).to_string();
                if req.starts_with("POST") {
                    if let Some(body)=req.split("\r\n\r\n").nth(1) {
                        let decoded=body.replace('+'," ").replace("%20"," ");
                        if let Ok(mut v)=shared_clip.lock() { v.push(format!("Dari HP: {}", decoded.trim())); }
                    }
                }
                let html=format!("<!doctype html><html><head><meta charset=utf-8><title>Venom Island</title></head><body><h1>Venom Island</h1><p>PIN: <b>{}</b></p><form method=POST><textarea name=text rows=4></textarea><button>Kirim ke PC</button></form></body></html>", pin);
                let resp=format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}", html.len(), html);
                let _=s.write_all(resp.as_bytes());
            }
        }
    });
}
pub struct VenomApp {
    pub settings: Settings,
    pub panel: Panel,
    pub show_radial: bool,
    pub island_center: Pos2,
    pub island_width: f32,
    pub target_width: f32,
    pub mouse_pos: Pos2,
    pub last_hover: Instant,
    pub is_hovering: bool,
    pub is_hidden: bool,
    pub blink_until: Instant,
    pub next_blink: Instant,
    pub battery: u8,
    pub charging: bool,
    pub cpu: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub volume: f32,
    pub brightness: f32,
    pub muted_speaker: bool,
    pub muted_mic: bool,
    pub is_playing: bool,
    pub track_title: String,
    pub track_artist: String,
    pub lyrics: Vec<String>,
    pub lyric_idx: usize,
    pub visualizer: Vec<f32>,
    pub vis_t: f32,
    pub prayer: PrayerTimes,
    pub prayer_last_fetch: Instant,
    pub prayer_countdown: String,
    pub selected_date: NaiveDate,
    pub tasks: HashMap<String, Vec<String>>,
    pub new_task: String,
    pub clipboard: Vec<(String, Instant)>,
    pub clipboard_input: String,
    pub file_shelf: Vec<PathBuf>,
    pub file_drag_hover: bool,
    pub pomodoro_secs_left: i64,
    pub pomodoro_running: bool,
    pub pomodoro_mode: String,
    pub pomodoro_last_tick: Instant,
    pub timer_secs_left: i64,
    pub timer_running: bool,
    pub timer_total: i64,
    pub timer_finished_jump: bool,
    pub timer_jump_until: Instant,
    pub teleprompter_text: String,
    pub teleprompter_playing: bool,
    pub awake: bool,
    pub focus_mode: bool,
    pub camera_on: bool,
    pub mic_on: bool,
    pub caps_on: bool,
    pub always_on_top: bool,
    pub wifi: String,
    pub traffic_down: u64,
    pub traffic_up: u64,
    pub download_files: Vec<String>,
    pub picked_color: Color32,
    pub show_power_confirm: String,
    pub phone_on: bool,
    pub phone_url: String,
    pub phone_shared: Arc<Mutex<Vec<String>>>,
    pub status_msg: String,
    pub status_until: Instant,
    pub sysinfo: sysinfo::System,
    pub last_stats: Instant,
    pub last_clip_check: Instant,
    pub export_text: String,
    pub gaming_blacklist: String,
    pub screenshot_mode: String,
    pub color_r: u8, pub color_g: u8, pub color_b: u8,
    pub calendar_month_offset: i32,
}
impl VenomApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let settings = Settings::load();
        let mut tasks: HashMap<String, Vec<String>> = HashMap::new();
        if let Ok(s) = std::fs::read_to_string("tasks.json") { if let Ok(v) = serde_json::from_str(&s) { tasks = v; } }
        let mut clipboard: Vec<(String, Instant)> = Vec::new();
        if let Ok(s) = std::fs::read_to_string("clipboard.json") { if let Ok(v) = serde_json::from_str::<Vec<String>>(&s) { for t in v { clipboard.push((t, Instant::now())); } } }
        let now = Instant::now();
        let phone_shared = Arc::new(Mutex::new(Vec::new()));
        Self {
            settings: settings.clone(), panel: Panel::None, show_radial: false,
            island_center: Pos2::new(490.0, 32.0), island_width: 240.0, target_width: 240.0,
            mouse_pos: Pos2::new(490.0, 32.0), last_hover: now, is_hovering: false, is_hidden: false,
            blink_until: now, next_blink: now + Duration::from_millis(2800),
            battery: 87, charging: false, cpu: 12.0, mem_used: 0, mem_total: 16000,
            volume: 65.0, brightness: 80.0, muted_speaker: false, muted_mic: false,
            is_playing: false, track_title: "Midnight Venom".to_string(), track_artist: "Venom Island".to_string(),
            lyrics: vec!["Putar lagu untuk melihat lirik dari lrclib.net".to_string()],
            lyric_idx: 0, visualizer: vec![0.3; 24], vis_t: 0.0,
            prayer: PrayerTimes::fallback(), prayer_last_fetch: now - Duration::from_secs(3600), prayer_countdown: "-".to_string(),
            selected_date: Local::now().date_naive(), tasks, new_task: String::new(),
            clipboard, clipboard_input: String::new(),
            file_shelf: Vec::new(), file_drag_hover: false,
            pomodoro_secs_left: 25*60, pomodoro_running: false, pomodoro_mode: "Fokus 25:00".to_string(), pomodoro_last_tick: now,
            timer_secs_left: 0, timer_running: false, timer_total: 300, timer_finished_jump: false, timer_jump_until: now,
            teleprompter_text: "Halo, ini Venom Island.\n\nBaca naskahmu dari sini.".to_string(), teleprompter_playing: false,
            awake: false, focus_mode: false, camera_on: false, mic_on: false, caps_on: false, always_on_top: settings.always_on_top,
            wifi: get_wifi_ssid(), traffic_down: 1240, traffic_up: 310, download_files: Vec::new(),
            picked_color: Color32::from_rgb(168,85,247),
            show_power_confirm: String::new(), phone_on: false, phone_url: "http://192.168.1.10:8080".to_string(), phone_shared,
            status_msg: "Venom Island siap. Hover kapsul untuk membangunkan.".to_string(), status_until: now + Duration::from_secs(4),
            sysinfo: sysinfo::System::new_all(), last_stats: now, last_clip_check: now,
            export_text: String::new(), gaming_blacklist: "valorant.exe, cs2.exe".to_string(), screenshot_mode: "Layar Penuh".to_string(),
            color_r: 168, color_g: 85, color_b: 247, calendar_month_offset: 0,
        }
    }
    pub fn set_status(&mut self, msg: &str) { self.status_msg = msg.to_string(); self.status_until = Instant::now() + Duration::from_secs(4); }
    pub fn save_tasks(&self) { let _ = std::fs::write("tasks.json", serde_json::to_string_pretty(&self.tasks).unwrap_or_default()); }
    pub fn save_clipboard(&self) { let v: Vec<String> = self.clipboard.iter().map(|(s,_)| s.clone()).collect(); let _ = std::fs::write("clipboard.json", serde_json::to_string_pretty(&v).unwrap_or_default()); }
    pub fn refresh_stats(&mut self) {
        if self.last_stats.elapsed() < Duration::from_millis(900) { return; }
        self.last_stats = Instant::now();
        self.sysinfo.refresh_all();
        // FIX E0599: sysinfo 0.30 tidak punya global_cpu_usage(), pakai global_cpu_info()
        self.cpu = self.sysinfo.global_cpu_info().cpu_usage();
        self.mem_used = self.sysinfo.used_memory() / 1024 / 1024;
        self.mem_total = self.sysinfo.total_memory() / 1024 / 1024;
        let (b,c) = get_battery(); self.battery=b; self.charging=c;
        self.caps_on = caps_lock_on();
        self.traffic_down = self.traffic_down.wrapping_add((self.cpu as u64)*3 + 12);
        self.traffic_up = self.traffic_up.wrapping_add((self.cpu as u64) + 5);
        self.wifi = get_wifi_ssid();
        self.update_prayer_countdown();
        self.scan_downloads();
        if let Ok(mut v) = self.phone_shared.try_lock() { for item in v.drain(..) { self.clipboard.insert(0,(item, Instant::now())); } }
        self.clipboard.retain(|(txt,t)| !(is_otp(txt) && t.elapsed() > Duration::from_secs(30)));
        if !self.clipboard.is_empty() { self.save_clipboard(); }
    }
    pub fn scan_downloads(&mut self) {
        let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
        let mut p = PathBuf::from(home); p.push("Downloads");
        if let Ok(rd) = std::fs::read_dir(p) {
            let mut v: Vec<String> = rd.flatten().filter_map(|e| e.file_name().into_string().ok()).collect();
            v.sort(); v.truncate(12); self.download_files = v;
        }
    }
    pub fn update_prayer_countdown(&mut self) {
        // FIX: tanpa and_local_timezone, hitung dari jam lokal langsung
        let now = Local::now();
        let now_mins = (now.hour() as i64)*60 + (now.minute() as i64);
        let list: Vec<(&str, &String)> = vec![("Imsak",&self.prayer.imsak),("Subuh",&self.prayer.subuh),("Dzuhur",&self.prayer.dzuhur),("Ashar",&self.prayer.ashar),("Maghrib",&self.prayer.maghrib),("Isya",&self.prayer.isya)];
        for (name, t) in list {
            if let Some((h,m)) = parse_hm(t) {
                let target = (h as i64)*60 + (m as i64);
                if target > now_mins {
                    let diff = target - now_mins;
                    self.prayer_countdown = format!("{} dalam {:02}:{:02}:00", name, diff/60, diff%60);
                    return;
                }
            }
        }
        self.prayer_countdown = "Isya selesai - menunggu Subuh besok".to_string();
    }
    // FIX clippy + E0382/E0502: pinjam, jangan ambil ownership
    pub fn toggle_panel(&mut self, p: &Panel) {
        if self.panel == *p { self.panel = Panel::None; self.show_radial = false; } else { self.panel = p.clone(); self.show_radial = true; }
        if self.settings.retro_sound { retro_beep(880, 60); }
    }
    pub fn theme(&self) -> Color32 { let c=self.settings.theme_color; Color32::from_rgba_unmultiplied(c[0],c[1],c[2],c[3]) }
}
impl eframe::App for VenomApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] { [0.0,0.0,0.0,0.0] }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) { self.mouse_pos = pos; }
        let scroll = ctx.input(|i| i.smooth_scroll_delta);
        if scroll.y != 0.0 && self.is_hovering {
            if self.mouse_pos.x > self.island_center.x {
                let d = if scroll.y > 0.0 { 2 } else { -2 };
                self.volume = (self.volume + d as f32).clamp(0.0,100.0);
                set_volume_delta(d); self.set_status(&format!("Volume {}%", self.volume as i32));
            } else {
                let d = if scroll.y > 0.0 { 3 } else { -3 };
                self.brightness = (self.brightness + d as f32).clamp(5.0,100.0);
                set_brightness(self.brightness as i32); self.set_status(&format!("Kecerahan {}% (laptop)", self.brightness as i32));
            }
        }
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        if !dropped.is_empty() {
            self.file_drag_hover = true;
            for d in dropped { if let Some(path) = d.path { self.file_shelf.push(path); } }
            self.panel = Panel::FileShelf; self.set_status("File masuk Rak File - Venom pusing!");
            if self.settings.retro_sound { retro_beep(220,120); }
        } else { self.file_drag_hover = false; }
        let dist = (self.mouse_pos - self.island_center).length();
        let hovering = dist < 220.0 && self.mouse_pos.y < 140.0;
        if hovering { self.is_hovering = true; self.last_hover = Instant::now(); self.is_hidden = false; }
        else if self.last_hover.elapsed() > Duration::from_secs_f32(self.settings.auto_hide_seconds.max(0.5)) && self.panel==Panel::None { self.is_hovering=false; self.is_hidden=true; }
        let sens = self.settings.cursor_sensitivity.clamp(0.2,2.0);
        let proximity = (1.0 - (dist / 280.0).clamp(0.0,1.0)) * sens;
        let base_w = if self.panel==Panel::Music { 520.0 } else if self.panel!=Panel::None { 420.0 } else { 240.0 };
        let elastic = if self.settings.animations { base_w + proximity*90.0 } else { base_w };
        self.target_width = elastic;
        self.island_width += (self.target_width - self.island_width) * if self.settings.animations { 0.18 } else { 1.0 };
        let now = Instant::now();
        if now >= self.next_blink { self.blink_until = now + Duration::from_millis(130); self.next_blink = now + Duration::from_millis(2600 + ((self.cpu as u64)%1800)); }
        self.vis_t += 0.12;
        if self.is_playing { for i in 0..self.visualizer.len() { self.visualizer[i] = 0.25 + 0.75*((self.vis_t + i as f32*0.7).sin().abs()); } }
        else { for v in self.visualizer.iter_mut() { *v *= 0.92; } }
        if self.pomodoro_running && self.pomodoro_last_tick.elapsed() >= Duration::from_secs(1) { self.pomodoro_last_tick = Instant::now(); self.pomodoro_secs_left -= 1; if self.pomodoro_secs_left<=0 { self.pomodoro_running=false; self.set_status("Pomodoro selesai! Istirahat."); } }
        if self.timer_running { self.timer_secs_left -= 1; if self.timer_secs_left<=0 { self.timer_running=false; self.timer_finished_jump=true; self.timer_jump_until=Instant::now()+Duration::from_secs(2); self.set_status("Timer selesai! Venom melompat kaget!"); } }
        if self.prayer_last_fetch.elapsed() > Duration::from_secs(3600) { self.prayer_last_fetch = Instant::now(); let city=self.settings.city.clone(); self.prayer = fetch_prayer_times(&city); }
        self.refresh_stats();
        if self.last_clip_check.elapsed() > Duration::from_secs(2) {
            self.last_clip_check = Instant::now();
            if let Ok(mut cb) = arboard::Clipboard::new() { if let Ok(txt) = cb.get_text() { let t=txt.trim().to_string(); if !t.is_empty() && !self.clipboard.iter().any(|(x,_)| *x==t) { self.clipboard.insert(0,(t, Instant::now())); if self.clipboard.len()>30 { self.clipboard.truncate(30); } } } }
        }
        let h = if self.panel!=Panel::None { 640.0 } else if self.show_radial { 280.0 } else { 72.0 };
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(980.0, h)));
        ctx.request_repaint_after(Duration::from_millis(33));
        let frame = egui::Frame { fill: Color32::TRANSPARENT, ..Default::default() };
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.set_min_size(Vec2::new(980.0, h));
            self.draw_island(ui);
            if self.show_radial || self.panel!=Panel::None { self.draw_radial(ui); }
            // FIX E0502: clone panel untuk match, jangan pinjam self.panel sambil pinjam self mut
            let cur = self.panel.clone();
            match cur {
                Panel::Settings => self.draw_settings(ui),
                Panel::Music => self.draw_music(ui),
                Panel::Calendar => self.draw_calendar(ui),
                Panel::Prayer => self.draw_prayer(ui),
                Panel::Clipboard => self.draw_clipboard(ui),
                Panel::FileShelf => self.draw_fileshelf(ui),
                Panel::Phone => self.draw_phone(ui),
                Panel::Teleprompter => self.draw_teleprompter(ui),
                Panel::Pomodoro => self.draw_pomodoro(ui),
                Panel::TaskManager => self.draw_taskmanager(ui),
                Panel::System => self.draw_system(ui),
                Panel::Power => self.draw_power(ui),
                Panel::Screenshot => self.draw_screenshot(ui),
                Panel::ColorPicker => self.draw_colorpicker(ui),
                Panel::Traffic => self.draw_traffic(ui),
                Panel::Support => self.draw_support(ui),
                Panel::Addons => self.draw_addons(ui),
                _ => {}
            }
            if Instant::now() < self.status_until {
                ui.label(egui::RichText::new(format!("● {}", self.status_msg)).small().color(Color32::LIGHT_GRAY));
            }
        });
    }
}
impl VenomApp {
    pub fn draw_island(&mut self, ui: &mut egui::Ui) {
        let w = self.island_width;
        let hidden_alpha = if self.is_hidden { 55u8 } else { 255u8 };
        let center_x = 490.0;
        let jump = if self.timer_finished_jump && Instant::now() < self.timer_jump_until { -12.0 } else { 0.0 };
        let top = 8.0 + jump;
        // FIX: pakai allocate_at_least (API stabil), bukan allocate_ui_at_rect
        let rect = Rect::from_min_size(Pos2::new(center_x - w/2.0, top), Vec2::new(w, 52.0));
        let (_alloc, resp) = ui.allocate_at_least(Vec2::new(980.0, 60.0), egui::Sense::click());
        let _ = _alloc;
        if resp.clicked() { self.show_radial = !self.show_radial; if !self.show_radial { self.panel = Panel::None; } if self.settings.retro_sound { retro_beep(660,50); } }
        self.island_center = rect.center();
        let painter = ui.painter();
        painter.rect_filled(rect, Rounding::same(26.0), Color32::from_black_alpha(40));
        let glass_a = ((235.0*self.settings.transparency) as u8).min(hidden_alpha);
        painter.rect_filled(rect, Rounding::same(26.0), Color32::from_rgba_unmultiplied(8,8,10,glass_a));
        painter.rect_stroke(rect, Rounding::same(26.0), Stroke::new(1.0, Color32::from_rgba_unmultiplied(255,255,255,28)));
        let eye_y = rect.center().y - 2.0;
        let is_blink = Instant::now() < self.blink_until;
        let is_hot = self.cpu > 80.0;
        let diff = self.mouse_pos - rect.center();
        let dir = if diff.length() > 0.1 { diff.normalized() * 4.0 } else { Vec2::ZERO };
        for side in [-1.0f32, 1.0] {
            let ex = rect.center().x + side * (w*0.14);
            let eye_w = 26.0; let eye_h = if is_blink { 3.0 } else { 22.0 };
            let eye_rect = Rect::from_center_size(Pos2::new(ex, eye_y), Vec2::new(eye_w, eye_h));
            painter.rect_filled(eye_rect, Rounding::same(10.0), Color32::WHITE);
            painter.circle_filled(eye_rect.center(), 14.0, Color32::from_rgba_unmultiplied(168,85,247,22));
            if !is_blink {
                let pupil = Pos2::new(eye_rect.center().x + dir.x, eye_rect.center().y + dir.y*0.6);
                painter.circle_filled(pupil, 5.0, Color32::BLACK);
                painter.circle_filled(Pos2::new(pupil.x+1.5, pupil.y-1.5), 1.6, Color32::WHITE);
            }
            painter.line_segment([Pos2::new(ex-14.0, eye_y-16.0), Pos2::new(ex+14.0, eye_y-10.0)], Stroke::new(2.5, Color32::from_rgb(200,200,200)));
        }
        if !is_blink {
            let mouth_y = rect.center().y + 16.0;
            painter.line_segment([Pos2::new(rect.center().x-18.0, mouth_y), Pos2::new(rect.center().x+18.0, mouth_y)], Stroke::new(1.2, Color32::from_rgba_unmultiplied(255,255,255,90)));
        }
        if is_hot {
            for (dx, dy) in [(-38.0, -18.0), (38.0, -18.0)] {
                painter.circle_filled(Pos2::new(rect.center().x+dx, rect.center().y+dy), 3.2, Color32::from_rgb(120,200,255));
            }
        }
        if self.file_drag_hover {
            painter.text(Pos2::new(rect.center().x, rect.min.y-10.0), egui::Align2::CENTER_BOTTOM, "@_@ pusing...", egui::FontId::proportional(10.0), Color32::YELLOW);
        }
        let bat_txt = if self.charging { format!("{}% ⚡", self.battery) } else if self.battery<=15 { format!("{}% 🪫", self.battery) } else if self.battery>=95 { format!("{}% 🔋 penuh", self.battery) } else { format!("{}%", self.battery) };
        let bat_color = if self.battery<=15 { Color32::RED } else if self.charging { Color32::GREEN } else { Color32::LIGHT_GRAY };
        painter.text(Pos2::new(rect.min.x+10.0, rect.center().y), egui::Align2::LEFT_CENTER, bat_txt.as_str(), egui::FontId::proportional(11.0), bat_color);
        let mut right_icons = String::new();
        if self.camera_on { right_icons.push_str("📷 "); } if self.mic_on { right_icons.push_str("🎙️ "); } if self.focus_mode { right_icons.push_str("👓 "); } if self.awake { right_icons.push_str("☕ "); } if self.caps_on { right_icons.push_str("⇪ "); } if self.is_playing { right_icons.push_str("🎵 "); }
        if !right_icons.is_empty() { painter.text(Pos2::new(rect.max.x-10.0, rect.center().y), egui::Align2::RIGHT_CENTER, right_icons.as_str(), egui::FontId::proportional(11.0), Color32::LIGHT_GRAY); }
        if self.is_hovering && self.panel==Panel::None {
            painter.text(Pos2::new(center_x, rect.max.y+10.0), egui::Align2::CENTER_TOP, "Klik untuk radial  •  Scroll kanan=Volume  kiri=Kecerahan", egui::FontId::proportional(10.0), Color32::from_rgba_unmultiplied(255,255,255,140));
        }
        if self.is_playing && self.panel==Panel::None {
            for i in 0..8 { let bx = rect.center().x - 28.0 + i as f32*8.0; let bh = self.visualizer.get(i).cloned().unwrap_or(0.3)*10.0; painter.rect_filled(Rect::from_min_size(Pos2::new(bx, rect.max.y-4.0-bh), Vec2::new(4.0,bh)), Rounding::same(2.0), self.theme()); }
        }
    }
    pub fn draw_radial(&mut self, ui: &mut egui::Ui) {
        let items: Vec<(&str, &str, Panel)> = vec![
            ("🎵","Musik",Panel::Music),("📸","Screenshot",Panel::Screenshot),("🎨","Warna",Panel::ColorPicker),("⚡","Daya",Panel::Power),
            ("📅","Kalender",Panel::Calendar),("🕌","Shalat",Panel::Prayer),("📋","Clipboard",Panel::Clipboard),("📁","Rak File",Panel::FileShelf),
            ("📱","HP",Panel::Phone),("💬","Teleprompter",Panel::Teleprompter),("⏰","Pomodoro",Panel::Pomodoro),("⚙️","Pengaturan",Panel::Settings),
            ("📊","Task Manager",Panel::TaskManager),("🌐","Traffic",Panel::Traffic),("🧩","Addons",Panel::Addons),("❤️","Dukung",Panel::Support),
        ];
        ui.horizontal_wrapped(|ui| {
            for (icon, tip, p) in items.iter() {
                let is_active = self.panel == *p;
                // FIX E0382: p tidak dipindah, kita pinjam &p ke toggle_panel
                if ui.button(format!("{} {}", icon, tip)).clicked() { let pp = p.clone(); self.toggle_panel(&pp); }
                if is_active { ui.label(egui::RichText::new("●").small().color(self.theme())); }
            }
        });
    }
    pub fn draw_settings(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("⚙️ Pengaturan Utama");
            ui.label("Tampilan, suara, sistem, ekspor/impor.");
            ui.separator();
            let mut anim = self.settings.animations; ui.checkbox(&mut anim, "Animasi membal elastis"); self.settings.animations = anim;
            let mut retro = self.settings.retro_sound; ui.checkbox(&mut retro, "Suara retro 8-bit"); self.settings.retro_sound = retro;
            let mut trans = self.settings.transparency; ui.add(egui::Slider::new(&mut trans, 0.4..=1.0).text("Transparansi kapsul")); self.settings.transparency = trans;
            let mut sens = self.settings.cursor_sensitivity; ui.add(egui::Slider::new(&mut sens, 0.2..=2.0).text("Sensitivitas kursor")); self.settings.cursor_sensitivity = sens;
            let mut hide = self.settings.auto_hide_seconds; ui.add(egui::Slider::new(&mut hide, 0.5..=5.0).text("Auto-sembunyi (detik)")); self.settings.auto_hide_seconds = hide;
            ui.horizontal(|ui| { ui.label("Warna tema:"); let mut col = self.picked_color; ui.color_edit_button_srgba(&mut col); self.picked_color = col; self.settings.theme_color = [col.r(), col.g(), col.b(), col.a()]; });
            let mut top = self.always_on_top; ui.checkbox(&mut top, "Kunci jendela selalu di atas"); self.always_on_top=top; self.settings.always_on_top=top;
            let mut gaming = self.settings.gaming_mode; ui.checkbox(&mut gaming, "Mode gaming"); self.settings.gaming_mode=gaming;
            ui.label("Daftar hitam game:"); let mut bl=self.gaming_blacklist.clone(); ui.text_edit_singleline(&mut bl); self.gaming_blacklist=bl;
            ui.separator();
            ui.label("Kota shalat:"); let mut city=self.settings.city.clone(); ui.text_edit_singleline(&mut city); self.settings.city=city;
            if ui.button("🔄 Ambil Jadwal Shalat").clicked() { let c=self.settings.city.clone(); self.prayer=fetch_prayer_times(&c); self.set_status("Jadwal shalat diperbarui"); }
            let mut pn=self.settings.prayer_notif; ui.checkbox(&mut pn, "Notifikasi shalat + bunyi Windows"); self.settings.prayer_notif=pn;
            let mut jawa=self.settings.show_jawa; ui.checkbox(&mut jawa, "Tampilkan pasaran Jawa"); self.settings.show_jawa=jawa;
            let mut hij=self.settings.show_hijri; ui.checkbox(&mut hij, "Tampilkan Hijriah"); self.settings.show_hijri=hij;
            ui.separator();
            let mut vol=self.volume; ui.add(egui::Slider::new(&mut vol, 0.0..=100.0).text("Volume")); self.volume=vol;
            let mut bri=self.brightness; ui.add(egui::Slider::new(&mut bri, 5.0..=100.0).text("Kecerahan (laptop)")); self.brightness=bri;
            if ui.button("🔈 Pengalih Output Suara (Windows)").clicked() { open_sound_settings(); }
            ui.separator();
            ui.label("Folder unduhan:"); let mut dl=self.settings.download_folder.clone(); ui.text_edit_singleline(&mut dl); self.settings.download_folder=dl;
            if ui.button("💾 Simpan Pengaturan").clicked() { self.settings.save(); self.set_status("Pengaturan disimpan"); }
            ui.horizontal(|ui| {
                if ui.button("📤 Ekspor .txt").clicked() { let txt=serde_json::to_string_pretty(&self.settings).unwrap_or_default(); self.export_text=txt.clone(); let _=std::fs::write("settings-export.txt", txt); self.set_status("Diekspor ke settings-export.txt"); }
                if ui.button("📥 Impor .txt").clicked() { if let Some(p)=rfd::FileDialog::new().add_filter("txt", &["txt"]).pick_file() { if let Ok(s)=std::fs::read_to_string(p) { if let Ok(v)=serde_json::from_str::<Settings>(&s) { self.settings=v; self.settings.save(); self.set_status("Pengaturan diimpor!"); } } } }
            });
            ui.text_edit_multiline(&mut self.export_text);
        });
    }
    pub fn draw_music(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎵 Mode Musik");
            ui.horizontal(|ui| { ui.label("Visualizer:"); for v in self.visualizer.iter().take(12) { let _=v; ui.label(egui::RichText::new("▮").color(self.theme())); } });
            ui.label(format!("{} - {}", self.track_title, self.track_artist));
            ui.horizontal(|ui| { ui.label("Judul:"); ui.text_edit_singleline(&mut self.track_title); ui.label("Artis:"); ui.text_edit_singleline(&mut self.track_artist); if ui.button("🔍 Ambil Lirik lrclib").clicked() { let a=self.track_artist.clone(); let t=self.track_title.clone(); self.lyrics=fetch_lyrics(&a,&t); self.lyric_idx=0; } });
            ui.horizontal(|ui| {
                if ui.button("⏮ Sebelumnya").clicked() { self.set_status("Lagu sebelumnya"); }
                let play_txt = if self.is_playing {"⏸ Jeda"} else {"▶ Putar"};
                if ui.button(play_txt).clicked() { self.is_playing=!self.is_playing; }
                if ui.button("⏭ Berikutnya").clicked() { self.set_status("Lagu berikutnya"); }
            });
            let mut vol=self.volume; ui.add(egui::Slider::new(&mut vol, 0.0..=100.0).text("Volume")); self.volume=vol;
            ui.label("📜 Lirik:");
            // FIX E0502: clone lyrics untuk iterasi, jangan pinjam self.lyrics sambil pinjam self mut di tombol
            let lyrics_clone = self.lyrics.clone();
            let cur_idx = self.lyric_idx;
            for (i,line) in lyrics_clone.iter().enumerate() { let is_cur=i==cur_idx; ui.label(egui::RichText::new(format!("{} {}", if is_cur {">"} else {" "}, line)).color(if is_cur {self.theme()} else {Color32::LIGHT_GRAY})); }
            ui.horizontal(|ui| { if ui.button("⬆ Baris Sebelumnya").clicked() { self.lyric_idx=self.lyric_idx.saturating_sub(1); } if ui.button("⬇ Baris Berikutnya").clicked() { if self.lyric_idx+1<self.lyrics.len() { self.lyric_idx+=1; } } });
        });
    }
    pub fn draw_calendar(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📅 Kalender — Masehi + Jawa + Hijriah");
            let today = Local::now().date_naive();
            let sel = self.selected_date;
            let (hy, hm, hd) = hijri_from_gregorian(sel);
            ui.label(format!("Dipilih: {}", sel.format("%A, %d %B %Y")));
            if self.settings.show_jawa { ui.label(format!("Pasaran: {} ({})", pasaran_jawa(sel), sel.format("%A"))); }
            if self.settings.show_hijri { ui.label(format!("Hijriah: {} {} {} H", hd, hm, hy)); }
            if let Some(h) = is_holiday_2026(sel) { ui.label(egui::RichText::new(format!("🔴 Tanggal Merah: {}", h)).color(Color32::RED)); }
            else if is_red_date(sel) { ui.label(egui::RichText::new("🔴 Minggu / Hari Libur").color(Color32::RED)); }
            ui.horizontal(|ui| {
                if ui.button("◀ Bulan Sebelumnya").clicked() { self.calendar_month_offset-=1; }
                if ui.button("Hari Ini").clicked() { self.selected_date=today; self.calendar_month_offset=0; }
                if ui.button("Bulan Berikutnya ▶").clicked() { self.calendar_month_offset+=1; }
            });
            let base_month = today.month() as i32 + self.calendar_month_offset;
            let mut yy = today.year(); let mut mm = base_month; while mm<1 { mm+=12; yy-=1; } while mm>12 { mm-=12; yy+=1; }
            ui.label(format!("Bulan: {}-{:02}", yy, mm as u32));
            // FIX E0502: kumpulkan klik dulu, baru ubah selected_date setelah loop
            let mut clicked_date: Option<NaiveDate> = None;
            ui.horizontal_wrapped(|ui| {
                for d in 1..=31 {
                    if let Some(date) = NaiveDate::from_ymd_opt(yy, mm as u32, d) {
                        let red = is_red_date(date);
                        if ui.button(format!("{}{}", d, if red {"*"} else {""})).clicked() { clicked_date = Some(date); }
                    }
                }
            });
            if let Some(d) = clicked_date { self.selected_date=d; self.set_status(&format!("Tanggal dipilih: {}", d)); }
            ui.label("* = tanggal merah");
            ui.separator(); ui.label(format!("📝 Tugas untuk {}", sel.format("%d %B %Y")));
            let key = sel.format("%Y-%m-%d").to_string();
            let tasks = self.tasks.get(&key).cloned().unwrap_or_default();
            let mut to_del: Option<usize> = None;
            for (i,t) in tasks.iter().enumerate() { ui.horizontal(|ui| { ui.label(format!("{}. {}", i+1, t)); if ui.button("🗑").clicked() { to_del = Some(i); } }); }
            if let Some(i) = to_del { if let Some(v)=self.tasks.get_mut(&key) { v.remove(i); } self.save_tasks(); }
            ui.horizontal(|ui| { ui.text_edit_singleline(&mut self.new_task); if ui.button("➕ Tambah Tugas").clicked() && !self.new_task.trim().is_empty() { let nt=self.new_task.trim().to_string(); self.tasks.entry(key.clone()).or_default().push(nt); self.new_task.clear(); self.save_tasks(); } });
        });
    }
    pub fn draw_prayer(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(format!("🕌 Jadwal Shalat — {} (WIB)", self.settings.city));
            if self.prayer.is_ramadan { ui.label(egui::RichText::new("🌙 Ramadan — Imsak & Berbuka ditandai.").color(Color32::from_rgb(255,200,80))); }
            ui.label(format!("⏳ Hitung mundur: {}", self.prayer_countdown));
            ui.label(format!("Hijriah: {}", self.prayer.hijri_date));
            let all = self.prayer.all();
            for (name, time) in all.iter() { ui.label(format!("{}: {}", name, time)); }
            ui.horizontal(|ui| { ui.label("Kota:"); let mut city=self.settings.city.clone(); ui.text_edit_singleline(&mut city); self.settings.city=city; if ui.button("Perbarui").clicked() { let c=self.settings.city.clone(); self.prayer=fetch_prayer_times(&c); } });
            let mut n=self.settings.prayer_notif; ui.checkbox(&mut n, "Pengingat bunyi Windows (on/off)"); self.settings.prayer_notif=n;
        });
    }
    pub fn draw_clipboard(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📋 Riwayat Clipboard — OTP auto-hapus 30 dtk");
            ui.horizontal(|ui| { ui.text_edit_singleline(&mut self.clipboard_input); if ui.button("➕ Simpan").clicked() && !self.clipboard_input.trim().is_empty() { let t=self.clipboard_input.trim().to_string(); self.clipboard.insert(0,(t, Instant::now())); self.clipboard_input.clear(); self.save_clipboard(); } if ui.button("🧹 Hapus Semua").clicked() { self.clipboard.clear(); self.save_clipboard(); } });
            let clip_clone = self.clipboard.clone();
            let mut to_del: Option<usize> = None;
            let mut to_copy: Option<String> = None;
            for (i,(txt,_)) in clip_clone.iter().enumerate() {
                let otp = is_otp(txt);
                let short: String = txt.chars().take(80).collect();
                ui.horizontal(|ui| {
                    ui.label(format!("{}{}", if otp {"🔐 OTP • "} else {""}, short));
                    if ui.button("📄 Salin").clicked() { to_copy = Some(txt.clone()); }
                    if ui.button("🗑").clicked() { to_del = Some(i); }
                });
            }
            if let Some(t) = to_copy { if let Ok(mut cb)=arboard::Clipboard::new() { let _=cb.set_text(t); self.set_status("Disalin ke clipboard"); } }
            if let Some(i) = to_del { self.clipboard.remove(i); self.save_clipboard(); }
        });
    }
    pub fn draw_fileshelf(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📁 Rak File — Tarik File ke Pulau");
            if ui.button("📂 Pilih File").clicked() { if let Some(p)=rfd::FileDialog::new().pick_file() { self.file_shelf.push(p); } }
            let shelf = self.file_shelf.clone();
            let mut to_del: Option<usize> = None;
            for (i,path) in shelf.iter().enumerate() {
                let name = path.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                ui.horizontal(|ui| { ui.label(name); if ui.button("🗑").clicked() { to_del = Some(i); } });
            }
            if let Some(i) = to_del { self.file_shelf.remove(i); }
        });
    }
    pub fn draw_phone(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📱 Sambung HP");
            ui.horizontal(|ui| { ui.label("URL:"); ui.text_edit_singleline(&mut self.phone_url); ui.label("PIN:"); let mut pin=self.settings.phone_pin.clone(); ui.text_edit_singleline(&mut pin); self.settings.phone_pin=pin; });
            if ui.button(if self.phone_on {"🛑 Matikan Server"} else {"▶ Aktifkan Server (port 8080)"}).clicked() {
                if !self.phone_on { let pin=self.settings.phone_pin.clone(); let shared=self.phone_shared.clone(); start_phone_server(pin, shared); self.phone_on=true; self.set_status("Server HP aktif di :8080"); } else { self.phone_on=false; }
            }
            ui.label("QR (scan dari HP):"); ui.monospace(qr_ascii(&self.phone_url));
        });
    }
    pub fn draw_teleprompter(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("💬 Teleprompter");
            let mut sp=self.settings.teleprompter_speed; ui.add(egui::Slider::new(&mut sp, 0.2..=3.0).text("Kecepatan")); self.settings.teleprompter_speed=sp;
            if ui.button(if self.teleprompter_playing {"⏸ Jeda"} else {"▶ Putar"}).clicked() { self.teleprompter_playing=!self.teleprompter_playing; }
            ui.text_edit_multiline(&mut self.teleprompter_text);
            ui.label(egui::RichText::new(self.teleprompter_text.clone()).heading());
        });
    }
    pub fn draw_pomodoro(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("⏰ Pomodoro / Timer");
            ui.label(format!("Pomodoro: {:02}:{:02} — {}", self.pomodoro_secs_left/60, self.pomodoro_secs_left%60, self.pomodoro_mode));
            ui.horizontal(|ui| {
                if ui.button(if self.pomodoro_running {"⏸ Jeda Pomodoro"} else {"▶ Mulai Pomodoro 25:00"}).clicked() { if !self.pomodoro_running { self.pomodoro_secs_left=25*60; self.pomodoro_mode="Fokus 25:00".to_string(); } self.pomodoro_running=!self.pomodoro_running; self.pomodoro_last_tick=Instant::now(); }
                if ui.button("☕ Istirahat 5:00").clicked() { self.pomodoro_secs_left=5*60; self.pomodoro_mode="Istirahat 5:00".to_string(); self.pomodoro_running=true; self.pomodoro_last_tick=Instant::now(); }
            });
            ui.label("Timer bebas:");
            // FIX E0308: tot i64, range harus i64 bukan f32
            let mut tot=self.timer_total; ui.add(egui::Slider::new(&mut tot, 10i64..=3600i64).text("Durasi timer (detik)")); self.timer_total=tot;
            ui.label(format!("Sisa timer: {} detik {}", self.timer_secs_left, if self.timer_finished_jump {"— Venom melompat kaget!"} else {""}));
            ui.horizontal(|ui| { if ui.button("▶ Mulai Timer").clicked() { self.timer_secs_left=self.timer_total; self.timer_running=true; } if ui.button("⏹ Stop").clicked() { self.timer_running=false; } });
        });
    }
    pub fn draw_taskmanager(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📊 Task Manager Mini");
            ui.label(format!("CPU: {:.1}% • RAM: {} / {} MB • Wi-Fi: {} • Baterai: {}%", self.cpu, self.mem_used, self.mem_total, self.wifi, self.battery));
            let mut procs: Vec<(String, f32, u64)> = self.sysinfo.processes().values().map(|pr| (pr.name().to_string_lossy().to_string(), pr.cpu_usage(), pr.memory()/1024/1024)).collect();
            procs.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)); procs.truncate(8);
            for (name, cpu, mem) in procs { ui.label(format!("{} — {:.1}% CPU • {} MB", name, cpu, mem)); }
        });
    }
    pub fn draw_system(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🖥️ Sistem");
            let mut awake=self.awake; ui.checkbox(&mut awake, "Awake — cegah tidur (☕ Venom minum kopi)"); self.awake=awake;
            let mut focus=self.focus_mode; ui.checkbox(&mut focus, "Mode Fokus — Venom berkacamata"); self.focus_mode=focus;
            if ui.button("🔒 Kunci Komputer").clicked() { lock_pc(); }
            ui.label(format!("Wi-Fi: {} • Caps Lock: {}", self.wifi, if self.caps_on {"ON"} else {"OFF"}));
            ui.label("Pelacak Unduhan:");
            let dl = self.download_files.clone();
            for f in dl.iter() { ui.label(format!("⬇ {}", f)); }
        });
    }
    pub fn draw_power(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("⚡ Daya");
            ui.label(format!("Baterai: {}% {}", self.battery, if self.charging {"⚡ Mengisi — Venom memegang charger"} else {""}));
            if !self.show_power_confirm.is_empty() {
                let act=self.show_power_confirm.clone();
                ui.label(format!("Yakin ingin {}?", act));
                ui.horizontal(|ui| {
                    if ui.button("✅ Ya").clicked() { match act.as_str() { "Tidur"=>sleep_pc(), "Restart"=>restart_pc(), "Matikan"=>shutdown_pc(), _=>{} } self.show_power_confirm.clear(); }
                    if ui.button("❌ Batal").clicked() { self.show_power_confirm.clear(); }
                });
            } else {
                ui.horizontal(|ui| {
                    if ui.button("💤 Tidur").clicked() { self.show_power_confirm="Tidur".to_string(); }
                    if ui.button("🔄 Restart").clicked() { self.show_power_confirm="Restart".to_string(); }
                    if ui.button("⏻ Matikan").clicked() { self.show_power_confirm="Matikan".to_string(); }
                    if ui.button("🔒 Kunci").clicked() { lock_pc(); }
                });
            }
            let mut vol=self.volume; ui.add(egui::Slider::new(&mut vol, 0.0..=100.0).text("Volume")); self.volume=vol;
            let mut bri=self.brightness; ui.add(egui::Slider::new(&mut bri, 5.0..=100.0).text("Kecerahan")); self.brightness=bri;
        });
    }
    pub fn draw_screenshot(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📸 Screenshot & Rekam Layar");
            ui.label("Mode:"); let mut m=self.screenshot_mode.clone(); ui.text_edit_singleline(&mut m); self.screenshot_mode=m;
            if ui.button("📸 Ambil Screenshot").clicked() { take_screenshot_full(); self.set_status("Screenshot diproses"); }
            if ui.button("🎬 Rekam Layar (Xbox Game Bar)").clicked() { run_powershell("Start-Process ms-gamingoverlay:"); }
        });
    }
    pub fn draw_colorpicker(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎨 Pipet Warna HEX / RGB");
            ui.horizontal(|ui| { let mut col=self.picked_color; ui.color_edit_button_srgba(&mut col); self.picked_color=col; ui.label(format!("HEX: #{:02X}{:02X}{:02X}", col.r(), col.g(), col.b())); ui.label(format!("RGB: {}, {}, {}", col.r(), col.g(), col.b())); });
            let mut r=self.color_r as i32; ui.add(egui::Slider::new(&mut r, 0..=255).text("R")); self.color_r=r as u8;
            let mut g=self.color_g as i32; ui.add(egui::Slider::new(&mut g, 0..=255).text("G")); self.color_g=g as u8;
            let mut b=self.color_b as i32; ui.add(egui::Slider::new(&mut b, 0..=255).text("B")); self.color_b=b as u8;
            self.picked_color=Color32::from_rgb(self.color_r, self.color_g, self.color_b);
            if ui.button("📋 Salin HEX").clicked() { let hex=format!("#{:02X}{:02X}{:02X}", self.color_r, self.color_g, self.color_b); if let Ok(mut cb)=arboard::Clipboard::new() { let _=cb.set_text(hex); } }
        });
    }
    pub fn draw_traffic(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🌐 Traffic Monitor");
            ui.label(format!("⬇ Unduh: {} MB • ⬆ Unggah: {} MB", self.traffic_down/10, self.traffic_up/10));
        });
    }
    pub fn draw_addons(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🧩 Addons");
            let mut a=self.settings.addons_prayer; ui.checkbox(&mut a, "Jadwal Shalat"); self.settings.addons_prayer=a;
            let mut b=self.settings.addons_traffic; ui.checkbox(&mut b, "Traffic Monitor"); self.settings.addons_traffic=b;
            if ui.button("💾 Simpan Addons").clicked() { self.settings.save(); self.set_status("Addons disimpan"); }
        });
    }
    pub fn draw_support(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("❤️ Dukung Kreator");
            ui.label("Trakteer (QRIS) — https://trakteer.id/venomisland");
            ui.monospace(qr_ascii("https://trakteer.id/venomisland"));
            ui.label("Ko-fi — https://ko-fi.com/venomisland");
            ui.label("Buy Me a Coffee — https://buymeacoffee.com/venomisland");
        });
    }
}
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(980.0, 72.0))
            .with_always_on_top()
            .with_transparent(true)
            .with_decorations(false)
            .with_resizable(false)
            .with_position(Pos2::new(480.0, 0.0)),
        ..Default::default()
    };
    eframe::run_native("Venom Island", options, Box::new(|cc| Box::new(VenomApp::new(cc))))
}
