#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

// Точка входа для операционной системы Android
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    use eframe::NativeOptions;

    let options = NativeOptions::default();

    // В eframe 0.34+ для Android вызывается run_native, куда передается app
    eframe::run_native(
        "ROBAS Android",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
    .expect("Ошибка запуска egui на Android");
}

// Обычная пустая функция main для проверок компилятора под десктоп
fn main() {}

struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: "Привет из Android!".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.heading("Приложение ROBAS");
        ui.text_edit_singleline(&mut self.text);
        ui.label(format!("Вы написали: {}", self.text));
    }
}
