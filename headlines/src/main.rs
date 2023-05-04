use eframe::{
    egui::{CentralPanel, ScrollArea},
    epi::App,
    run_native, NativeOptions,
};

struct Headlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl Headlines {
    fn new() -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://google.com?i={}", a),
        });
        Self {
            articles: Vec::from_iter(iter),
        }
    }
}

// Main container
impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            // create a scrollable ui widget
            ScrollArea::auto_sized().show(ui, |ui| {
                for a in &self.articles {
                    ui.label(&a.title);
                    ui.label(&a.desc);
                    ui.label(&a.url);
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app = Headlines::new();
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}
