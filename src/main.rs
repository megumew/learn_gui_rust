mod mpps;


use eframe::{
    egui::{CentralPanel, ScrollArea, Vec2, Ui, Separator, TopBottomPanel, Label, CtxRef, RichText, Hyperlink,},
    epi::App,
    run_native, NativeOptions,
};
use mpps::{Mpps, PADDING};


impl App for Mpps {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::new([false, true]).show(ui, |ui| {
                self.render_members(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "mpps"
    }
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui|{
        ui.vertical_centered(|ui|{
            ui.add_space(10.0);
            ui.add(Hyperlink::from_label_and_url("made by megu", "https://github.com/megumew"));
        })
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("mpps");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.0);
    ui.add(sep);
}

fn main() {
    let app = Mpps::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(1000.0, 720.0));
    run_native(Box::new(app), win_option);
}
