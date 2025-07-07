use calendar::CalendarApp;

pub fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Calendar",
        native_options,
        Box::new(|cc| Ok(Box::new(CalendarApp::new(cc)))),
    )
}
