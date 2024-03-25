use std::cmp::Ordering;

use crate::search::setup_database;
use crate::search::Database;
use crate::search::School;

const OUR_COUNTRY: &str = "United States";

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
#[serde(default)]
struct Settings {
    dark_mode: bool,
}

/// Data Persistence
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    settings: Settings,
    query: String,
    last_query: String,
    results: Vec<School>,
    #[serde(skip)]
    database: Database,
}

impl Default for App {
    fn default() -> Self {
        let database = setup_database();
        let results = database.query_documents("");
        let settings = Default::default();
        Self {
            settings,
            query: String::new(),
            results,
            last_query: String::new(),
            database,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Save app data
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            query,
            database,
            ref mut settings,
            ..
        } = self;
        let updated: bool = *query != self.last_query;

        // Visuals
        if settings.dark_mode {
            ctx.set_visuals(egui::Visuals::light());
        } else {
            ctx.set_visuals(egui::Visuals::dark());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Menu Bar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Toggle Theme").clicked() {
                        settings.dark_mode = !settings.dark_mode;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.heading("School Search");
            ui.horizontal(|ui| {
                ui.label("Version:");
                ui.label(egui::RichText::new(env!("CARGO_PKG_VERSION")).color(egui::Color32::RED));
            });
            let link = egui::Hyperlink::from_label_and_url(
                "Author".to_string(),
                "https://github.com/AceofSpades5757".to_string(),
            );
            ui.add(link);
            ui.add_space(15.0);

            // Search Bar
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Search:").strong().size(16.0));
                ui.text_edit_singleline(query);
            });
            ui.add(egui::widgets::Separator::default());

            // Search Results
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                if updated {
                    let mut results = database.query_documents(query);
                    results.sort_by(|school_1, school_2| {
                        if school_1.country == OUR_COUNTRY {
                            Ordering::Less
                        } else if school_2.country == OUR_COUNTRY {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    });
                    self.results = results;
                    self.last_query = query.to_string();
                }
                for result in &self.results {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Name:").strong().size(16.0));
                        ui.label(&result.name);
                    });

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Country:").strong().size(16.0));
                        ui.label(&result.country);
                    });

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Web Sites:").strong().size(16.0)
                        );
                        for site in &result.web_pages {
                            let link = egui::Hyperlink::from_label_and_url(
                                site.to_string(),
                                site.to_string(),
                            );
                            ui.add(link);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Domains:").strong().size(16.0));
                        ui.label(&result.domains.join(", "));
                    });

                    ui.add(egui::widgets::Separator::default());
                }
            });

            // Footer
            egui::warn_if_debug_build(ui);
        });
    }
}
