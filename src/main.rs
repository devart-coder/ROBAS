use ROBAS::application::Application;

fn main() -> eframe::Result<(), eframe::Error> {
    let default_native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "obas",
        default_native_options,
        Box::new(|cc| {
            let mut visuals = egui::Visuals::dark();

            // Задаем тот же (или любой нужный) цвет фона для CentralPanel
            visuals.panel_fill = egui::Color32::from_rgb(30, 30, 35);

            cc.egui_ctx.set_visuals(visuals);
            Ok(Box::new(Application::default()))
        }),
    )
}
