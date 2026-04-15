use chrono::{Datelike, Local};
use eframe::egui::{self, Color32, RichText, Stroke};
use image::io::Reader as ImageReader;
use rand::Rng;
use std::env;
use std::path::Path;
use std::time::{Duration, Instant};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ThermoOS - ReptileSim",
        options,
        Box::new(|cc| Box::new(ThermoApp::new(cc))),
    )
}

#[derive(Clone)]
struct Zone {
    name: &'static str,
    target_temp: (f32, f32),
    target_humidity: (u8, u8),
    temp: f32,
    humidity: u8,
    uv_a_on: bool,
    uv_b_on: bool,
    heat_on: bool,
    pump_on: bool,
    co2_ppm: u16,
    lux_on: bool,
    kelvin_6500_on: bool,
    color: Color32,
    day_runtime: &'static str,
    night_runtime: &'static str,
    sensor_online: bool,
}

impl Zone {
    fn status_temp(&self) -> &'static str {
        if self.temp < self.target_temp.0 {
            "SOUS CIBLE"
        } else if self.temp > self.target_temp.1 {
            "SUR CIBLE"
        } else {
            "OK"
        }
    }

    fn status_humidity(&self) -> &'static str {
        if self.humidity < self.target_humidity.0 {
            "SEC"
        } else if self.humidity > self.target_humidity.1 {
            "HUMIDE"
        } else {
            "OK"
        }
    }

    fn display_temp(&self) -> String {
        if self.sensor_online {
            format!("{:.1}°C", self.temp)
        } else {
            "--.-°C".to_owned()
        }
    }

    fn display_humidity(&self) -> String {
        if self.sensor_online {
            format!("{}%", self.humidity)
        } else {
            "---%".to_owned()
        }
    }
}

#[derive(Clone)]
struct ReptileInfo {
    espece: &'static str,
    sexe: &'static str,
    age: &'static str,
    regime: &'static str,
    cites: &'static str,
    naissance: &'static str,
    poids: &'static str,
    taille: &'static str,
    mutation: &'static str,
    eleveur: &'static str,
    micropuce: &'static str,
    alimentation: &'static str,
    acquisition: &'static str,
    notes: &'static str,
}

#[derive(Clone)]
struct AstroCycle {
    sun: &'static str,
    moon: &'static str,
    saison: &'static str,
    hibernation: &'static str,
    uv_index: &'static str,
    meteo: &'static str,
}

#[derive(Clone)]
struct SystemState {
    rain_bar: bool,
    ventilation: bool,
    wifi: bool,
    bluetooth: bool,
    battery_percent: u8,
}

struct ThermoApp {
    zones: Vec<Zone>,
    reptile: ReptileInfo,
    astro: AstroCycle,
    system: SystemState,
    preview_texture: Option<egui::TextureHandle>,
    preview_status: String,
    last_tick: Instant,
}

impl ThermoApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let (preview_texture, preview_status) = load_preview_texture(&cc.egui_ctx);

        Self {
            zones: vec![
                Zone {
                    name: "zone chaude",
                    target_temp: (30.0, 35.0),
                    target_humidity: (30, 50),
                    temp: 31.8,
                    humidity: 41,
                    uv_a_on: false,
                    uv_b_on: false,
                    heat_on: false,
                    pump_on: false,
                    co2_ppm: 510,
                    lux_on: true,
                    kelvin_6500_on: false,
                    color: Color32::from_rgb(255, 168, 38),
                    day_runtime: "--/--",
                    night_runtime: "--/--",
                    sensor_online: true,
                },
                Zone {
                    name: "zone intermédiaire",
                    target_temp: (24.0, 29.0),
                    target_humidity: (45, 65),
                    temp: 26.2,
                    humidity: 52,
                    uv_a_on: false,
                    uv_b_on: false,
                    heat_on: false,
                    pump_on: false,
                    co2_ppm: 560,
                    lux_on: true,
                    kelvin_6500_on: false,
                    color: Color32::from_rgb(255, 214, 64),
                    day_runtime: "--/--",
                    night_runtime: "--/--",
                    sensor_online: true,
                },
                Zone {
                    name: "zone humide",
                    target_temp: (20.0, 25.0),
                    target_humidity: (70, 90),
                    temp: 23.0,
                    humidity: 76,
                    uv_a_on: false,
                    uv_b_on: false,
                    heat_on: false,
                    pump_on: false,
                    co2_ppm: 610,
                    lux_on: true,
                    kelvin_6500_on: false,
                    color: Color32::from_rgb(33, 212, 253),
                    day_runtime: "--/--",
                    night_runtime: "--/--",
                    sensor_online: true,
                },
                Zone {
                    name: "bassin",
                    target_temp: (20.0, 26.0),
                    target_humidity: (90, 100),
                    temp: 24.4,
                    humidity: 93,
                    uv_a_on: false,
                    uv_b_on: false,
                    heat_on: false,
                    pump_on: true,
                    co2_ppm: 590,
                    lux_on: true,
                    kelvin_6500_on: false,
                    color: Color32::from_rgb(140, 255, 229),
                    day_runtime: "--/--",
                    night_runtime: "--/--",
                    sensor_online: true,
                },
            ],
            reptile: ReptileInfo {
                espece: "Dragon barbu",
                sexe: "inconnu",
                age: "--",
                regime: "Diurne",
                cites: "Annexe B (à vérifier)",
                naissance: "--",
                poids: "--",
                taille: "--",
                mutation: "--",
                eleveur: "--",
                micropuce: "--",
                alimentation: "--",
                acquisition: "--",
                notes: "--",
            },
            astro: AstroCycle {
                sun: "06:32 → 19:47",
                moon: "Croissant (42%)",
                saison: "Printemps",
                hibernation: "25 j restant",
                uv_index: "7",
                meteo: "Wi‑Fi indisponible (hors-ligne)",
            },
            system: SystemState {
                rain_bar: false,
                ventilation: false,
                wifi: true,
                bluetooth: true,
                battery_percent: 86,
            },
            preview_texture,
            preview_status,
            last_tick: Instant::now(),
        }
    }

    fn simulate(&mut self) {
        let mut rng = rand::thread_rng();
        for zone in &mut self.zones {
            if rng.gen_bool(0.01) {
                zone.sensor_online = !zone.sensor_online;
            }
            if !zone.sensor_online {
                continue;
            }

            let d_temp: f32 = rng.gen_range(-0.12..=0.12);
            zone.temp =
                (zone.temp + d_temp).clamp(zone.target_temp.0 - 4.0, zone.target_temp.1 + 4.0);

            let d_h: i16 = rng.gen_range(-2..=2);
            let new_h = (zone.humidity as i16 + d_h).clamp(
                (zone.target_humidity.0 as i16) - 15,
                (zone.target_humidity.1 as i16) + 15,
            );
            zone.humidity = new_h as u8;

            zone.heat_on = zone.temp < zone.target_temp.0;
            zone.pump_on = zone.name == "bassin" && zone.humidity < 92;
            zone.kelvin_6500_on = zone.lux_on && !zone.uv_b_on;
            zone.co2_ppm = (zone.co2_ppm as i16 + rng.gen_range(-8..=8)).clamp(350, 1200) as u16;
        }

        self.system.ventilation = self.zones.iter().any(|z| z.co2_ppm > 900);
        if rng.gen_bool(0.03) {
            self.system.wifi = !self.system.wifi;
        }
        self.astro.meteo = if self.system.wifi {
            "Partiellement nuageux 24°C"
        } else {
            "Wi‑Fi indisponible (hors-ligne)"
        };
    }

    fn draw_zone(ui: &mut egui::Ui, zone: &mut Zone) {
        egui::Frame::none()
            .stroke(Stroke::new(1.0, Color32::from_gray(55)))
            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
            .show(ui, |ui| {
                let card_rect = ui.max_rect();
                let accent = egui::Rect::from_min_max(
                    card_rect.left_top(),
                    egui::pos2(card_rect.left() + 4.0, card_rect.bottom()),
                );
                ui.painter().rect_filled(accent, 2.0, zone.color);

                ui.horizontal(|ui| {
                    ui.colored_label(zone.color, RichText::new(zone.name).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(zone.display_humidity())
                                .size(28.0)
                                .color(Color32::from_rgb(126, 217, 255))
                                .strong(),
                        );
                        ui.label("   ");
                        ui.label(
                            RichText::new(zone.display_temp())
                                .size(28.0)
                                .color(Color32::WHITE)
                                .strong(),
                        );
                    });
                });
                ui.small(format!(
                    "Cible {:.0}-{:.0}°C / {}-{}%",
                    zone.target_temp.0,
                    zone.target_temp.1,
                    zone.target_humidity.0,
                    zone.target_humidity.1
                ));
                ui.horizontal_wrapped(|ui| {
                    tag_pill(ui, "ÉCLAIRAGE", zone.lux_on, false);
                    tag_pill(ui, "UVA", zone.uv_a_on, true);
                    tag_pill(ui, "UVB", zone.uv_b_on, true);
                    tag_pill(ui, "6500K", zone.kelvin_6500_on, false);
                    tag_pill(ui, "CHAUF", zone.heat_on, true);
                    tag_pill(ui, "POMPE", zone.pump_on, true);
                });
                ui.label(format!(
                    "Temp: {} | Hygro: {} | CO₂: {}",
                    zone.status_temp(),
                    zone.status_humidity(),
                    if zone.sensor_online {
                        format!("{} ppm", zone.co2_ppm)
                    } else {
                        "--- ppm".to_owned()
                    }
                ));
                ui.small(format!(
                    "Jour {}  •  Nuit {}",
                    zone.day_runtime, zone.night_runtime
                ));
                if !zone.sensor_online {
                    ui.colored_label(Color32::from_rgb(255, 178, 178), "Capteur hors-ligne");
                }
            });
    }
}

impl eframe::App for ThermoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.last_tick.elapsed() >= Duration::from_millis(1200) {
            self.simulate();
            self.last_tick = Instant::now();
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.columns(3, |cols| {
                cols[0].label(format_local_fr(Local::now()));
                cols[1].with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.heading(
                            RichText::new("ThermoOS").color(Color32::from_rgb(126, 217, 255)),
                        );
                    },
                );
                cols[2].with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("⚙");
                    ui.label(if self.system.wifi { "📶" } else { "📴" });
                    ui.label(format!("🔋{}%", self.system.battery_percent));
                    ui.label(if self.system.bluetooth { "◉" } else { "○" });
                    ui.label("🔔");
                });
            });
        });

        egui::SidePanel::left("left")
            .resizable(true)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.label(RichText::new("APERÇU ANIMAL").strong());
                    ui.add_space(8.0);
                    let image_height = 160.0;
                    let image_size = egui::vec2(ui.available_width(), image_height);

                    if let Some(texture) = &self.preview_texture {
                        let image = egui::Image::new(texture).fit_to_exact_size(image_size);
                        ui.add(image);
                        ui.small("Source: REPTILE_PREVIEW_PATH");
                    } else {
                        let (rect, _) = ui.allocate_exact_size(image_size, egui::Sense::hover());
                        ui.painter()
                            .rect_filled(rect, 6.0, Color32::from_rgb(58, 58, 58));
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Flux caméra non configuré",
                            egui::FontId::proportional(16.0),
                            Color32::LIGHT_GRAY,
                        );
                        ui.small(&self.preview_status);
                    }
                });

                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.label(RichText::new("ASTRO & CYCLES").strong());
                    ui.columns(2, |cols| {
                        cols[0].colored_label(Color32::from_rgb(255, 212, 97), "SOLEIL");
                        cols[1].colored_label(Color32::from_rgb(126, 217, 255), "LUNE");
                        cols[0].label(self.astro.sun);
                        cols[1].label(self.astro.moon);
                    });
                    let uv_index = self.astro.uv_index.parse::<f32>().unwrap_or(0.0);
                    ui.add(
                        egui::ProgressBar::new((uv_index / 11.0).clamp(0.0, 1.0))
                            .text(format!("UV INDEX {}", self.astro.uv_index))
                            .fill(Color32::from_rgb(255, 214, 64)),
                    );
                    ui.columns(2, |cols| {
                        cols[0].label(format!("SAISON\n{}", self.astro.saison));
                        cols[1].label(format!("BRUMATION\n{}", self.astro.hibernation));
                    });
                    ui.small(format!("Météo: {}", self.astro.meteo));
                });
            });

        egui::SidePanel::right("right")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("✓ INFORMATIONS REPTILE")
                        .strong()
                        .color(Color32::LIGHT_GREEN),
                );
                key_val(ui, "Espèce", self.reptile.espece);
                key_val(ui, "Sexe", self.reptile.sexe);
                key_val(ui, "Âge", self.reptile.age);
                key_val(ui, "Naissance", self.reptile.naissance);
                key_val(ui, "Régime", self.reptile.regime);
                key_val(ui, "Poids", self.reptile.poids);
                key_val(ui, "Taille", self.reptile.taille);
                key_val(ui, "Mutation", self.reptile.mutation);
                key_val(ui, "CITES", self.reptile.cites);
                key_val(ui, "Éleveur", self.reptile.eleveur);
                key_val(ui, "Micropuce", self.reptile.micropuce);
                key_val(ui, "Alimentation", self.reptile.alimentation);
                key_val(ui, "Acquisition", self.reptile.acquisition);
                key_val(ui, "Notes", self.reptile.notes);

                ui.separator();
                ui.label(RichText::new("SYSTÈME").strong());
                key_val(
                    ui,
                    "Rain bar",
                    if self.system.rain_bar { "ON" } else { "OFF" },
                );
                key_val(
                    ui,
                    "Ventilation",
                    if self.system.ventilation { "ON" } else { "OFF" },
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                ThermoApp::draw_zone(&mut columns[0], &mut self.zones[0]);
                ThermoApp::draw_zone(&mut columns[1], &mut self.zones[1]);
            });
            ui.add_space(6.0);
            ui.columns(2, |columns| {
                ThermoApp::draw_zone(&mut columns[0], &mut self.zones[2]);
                ThermoApp::draw_zone(&mut columns[1], &mut self.zones[3]);
            });
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn tag_pill(ui: &mut egui::Ui, label: &str, on: bool, critical: bool) {
    let (bg, fg) = if on {
        if critical {
            (
                Color32::from_rgb(42, 104, 76),
                Color32::from_rgb(133, 255, 197),
            )
        } else {
            (
                Color32::from_rgb(43, 95, 104),
                Color32::from_rgb(141, 233, 255),
            )
        }
    } else {
        (
            Color32::from_rgb(72, 62, 62),
            Color32::from_rgb(255, 178, 178),
        )
    };
    let text = if on {
        format!("{label} ON")
    } else {
        format!("{label} OFF")
    };
    egui::Frame::none()
        .fill(bg)
        .rounding(6.0)
        .inner_margin(egui::Margin::symmetric(8.0, 2.0))
        .show(ui, |ui| {
            ui.label(RichText::new(text).color(fg).strong());
        });
}

fn key_val(ui: &mut egui::Ui, k: &str, v: &str) {
    ui.horizontal(|ui| {
        ui.colored_label(Color32::GRAY, k);
        let value_color = match v {
            "ON" => Color32::LIGHT_GREEN,
            "OFF" => Color32::LIGHT_RED,
            _ => Color32::WHITE,
        };
        ui.colored_label(value_color, v);
    });
}

fn load_preview_texture(ctx: &egui::Context) -> (Option<egui::TextureHandle>, String) {
    let var_name = "REPTILE_PREVIEW_PATH";
    let path_value = match env::var(var_name) {
        Ok(v) if !v.trim().is_empty() => v,
        _ => return (None, format!("Définir {var_name}=/chemin/vers/image.jpg")),
    };

    let path = Path::new(&path_value);
    let reader = match ImageReader::open(path) {
        Ok(r) => r,
        Err(err) => return (None, format!("Impossible d'ouvrir {path_value}: {err}")),
    };

    let decoded = match reader.decode() {
        Ok(img) => img,
        Err(err) => return (None, format!("Décodage image impossible: {err}")),
    };

    let rgba = decoded.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    let texture = ctx.load_texture("reptile-preview", color_image, egui::TextureOptions::LINEAR);
    (Some(texture), format!("Image chargée: {path_value}"))
}

fn format_local_fr(now: chrono::DateTime<Local>) -> String {
    let weekday = match now.weekday().num_days_from_monday() {
        0 => "lun",
        1 => "mar",
        2 => "mer",
        3 => "jeu",
        4 => "ven",
        5 => "sam",
        _ => "dim",
    };
    let month = match now.month() {
        1 => "jan",
        2 => "fév",
        3 => "mar",
        4 => "avr",
        5 => "mai",
        6 => "jun",
        7 => "jul",
        8 => "aoû",
        9 => "sep",
        10 => "oct",
        11 => "nov",
        _ => "déc",
    };
    format!(
        "{weekday} {:02} {month}  {}",
        now.day(),
        now.format("%H:%M")
    )
}
