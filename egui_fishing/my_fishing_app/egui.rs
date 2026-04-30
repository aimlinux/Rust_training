use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "釣果報告アプリ",
        options,
        Box::new(|_cc| Ok(Box::new(FishingApp::default()))),
    )
}

struct Report {
    fish: String,
    size: String,
    place: String,
}

struct FishingApp {
    // 入力用フィールド
    input_fish: String,
    input_size: String,
    input_place: String,
    // 報告リスト
    reports: Vec<Report>,
}

impl Default for FishingApp {
    fn default() -> Self {
        Self {
            input_fish: String::new(),
            input_size: String::new(),
            input_place: String::new(),
            reports: Vec::new(),
        }
    }
}

impl eframe::App for FishingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎣 釣果報告");

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("魚種:");
                    ui.text_edit_singleline(&mut self.input_fish);
                });

                ui.horizontal(|ui| {
                    ui.label("サイズ (cm):");
                    ui.text_edit_singleline(&mut self.input_size);
                });

                ui.horizontal(|ui| {
                    ui.label("場所:");
                    ui.text_edit_singleline(&mut self.input_place);
                });

                if ui.button("報告を追加").clicked() {
                    if !self.input_fish.is_empty() {
                        self.reports.push(Report {
                            fish: self.input_fish.clone(),
                            size: self.input_size.clone(),
                            place: self.input_place.clone(),
                        });
                        // 入力をリセット
                        self.input_fish.clear();
                        self.input_size.clear();
                        self.input_place.clear();
                    }
                }
            });

            ui.separator();

            ui.heading("記録一覧");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for r in &self.reports {
                    ui.label(format!("🐟 {} / {}cm / 📍{}", r.fish, r.size, r.place));
                }
            });
        });
    }
}