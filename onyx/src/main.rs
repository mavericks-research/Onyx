mod ui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GitHub Repo Dashboard",
        native_options,
        Box::new(|_cc| Box::new(ui::dashboard::DashboardApp::default())),
    )
}
