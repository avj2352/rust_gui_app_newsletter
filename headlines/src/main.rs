use eframe::{
    egui::{CentralPanel, CtxRef, FontDefinitions, FontFamily, ScrollArea, Vec2},
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

    fn configure_fonts(&self, ctx: &CtxRef) {
        // create font definition
        let mut font_def = FontDefinitions::default();
        // load font
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            std::borrow::Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        );
        // set size
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );
        // tell egui to load our custom font
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());
        // load the font using contxt object
        ctx.set_fonts(font_def);
    }
}

// Main container
impl App for Headlines {
    // customize fonts
    fn setup(
        &mut self,
        _ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(_ctx);
    }

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
    let mut win_option = NativeOptions::default();
    // set window size
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_option);
}
