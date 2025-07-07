use eframe::egui::{self, Key};

use chrono::{Datelike, NaiveDate, Weekday};

#[derive(Default)]
pub struct CalendarApp {
    year: i32,
    month: u32,
}

impl CalendarApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn default() -> Self {
        let today = chrono::Local::now();
        Self {
            year: today.year(),
            month: today.month(),
        }
    }
}

impl eframe::App for CalendarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let input = ctx.input(|i| i.clone());
        let is_quit = input.modifiers.command && input.key_pressed(Key::Q);

        if is_quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.small_button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    if self.month == 1 {
                        self.month = 12;
                        self.year -= 1;
                    } else {
                        self.month -= 1;
                    }
                }

                ui.label(format!("{} {}", self.month, self.year));

                if ui.button(">").clicked() {
                    if self.month == 12 {
                        self.month = 1;
                        self.year += 1;
                    } else {
                        self.month += 1;
                    }
                }
            });

            let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
            egui::Grid::new("calendar_header")
                .num_columns(7)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {
                    for &wd in &weekdays {
                        ui.label(wd);
                    }

                    ui.end_row();
                });

            let first =
                NaiveDate::from_ymd_opt(self.year, self.month, 1).expect("valid year/month");

            let next_month = if self.month == 12 {
                NaiveDate::from_ymd_opt(self.year + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(self.year, self.month + 1, 1)
            }
            .expect("valid next month");

            let last_day = next_month.pred_opt().expect("next_month > 1st of month");

            let offset = match first.weekday() {
                Weekday::Mon => 0,
                Weekday::Tue => 1,
                Weekday::Wed => 2,
                Weekday::Thu => 3,
                Weekday::Fri => 4,
                Weekday::Sat => 5,
                Weekday::Sun => 6,
            };
            let days_in_month = last_day.day();

            let total_cells = ((offset as u32 + days_in_month + 6) / 7) * 7;

            egui::Grid::new("calendar_days")
                .num_columns(7)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {
                    let mut day = 1;

                    for cell in 0..total_cells {
                        if cell < offset as u32 || day > days_in_month {
                            ui.label(" ");
                        } else {
                            if ui.button(day.to_string()).clicked() {
                                println!("Clicked on day {}", day);
                            }
                            day += 1;
                        }
                        if (cell + 1) % 7 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("App is closed");
    }
}
