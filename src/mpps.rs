use std::borrow::Cow;

use eframe::egui::{
    self, Button, Color32, CtxRef, FontData, FontDefinitions, FontFamily, Label, Layout, RichText,
    Separator, TopBottomPanel, Ui,
};

pub const PADDING: f32 = 5.0;
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

pub struct Mpps {
    members: Vec<Member>,
}

impl Mpps {
    pub fn new() -> Mpps {
        let iter = (0..5).map(|a| Member {
            name: format!("Person{}", a),
            expenses: format!("expenses:\n1: {}\n2: {}\n3: {}", a, a, a),
        });
        Mpps {
            members: Vec::from_iter(iter),
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "Roboto".to_string(),
            FontData {
                index: 0,
                font: Cow::Borrowed(include_bytes!("Roboto-Bold.ttf")),
            },
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Monospace, 35.0),
        );
        font_def
            .family_and_size
            .insert(eframe::egui::TextStyle::Body, (FontFamily::Monospace, 15.0));
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "Roboto".to_string());
        ctx.set_fonts(font_def);
    }

    pub fn render_members(&self, ui: &mut Ui) {
        for a in &self.members {
            ui.add_space(PADDING);
            //render name
            let name =
                Label::new(RichText::new(&a.name).text_style(eframe::egui::TextStyle::Heading));
            ui.add(name);

            //render expenses
            ui.add_space(PADDING);
            let expenses =
                Label::new(RichText::new(&a.expenses).text_style(eframe::egui::TextStyle::Button));
            ui.add(expenses);

            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub fn render_top_panel(&self, ctx: &CtxRef) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.0);
            egui::menu::bar(ui, |ui| {
                //logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new(
                        RichText::new("🐱").text_style(egui::TextStyle::Heading),
                    ));
                });
                //controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new(
                        RichText::new("❌").text_style(egui::TextStyle::Body),
                    ));
                    let refresh_btn = ui.add(Button::new(
                        RichText::new("🔁").text_style(egui::TextStyle::Body),
                    ));
                    let theme_btn = ui.add(Button::new(
                        RichText::new("🌙").text_style(egui::TextStyle::Body),
                    ));
                });
            });
            ui.add_space(10.0);
        });
    }
}

pub struct Member {
    name: String,
    expenses: String,
}
