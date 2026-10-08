// main focus rn: make the core ui, not functionality
// use helper functions

use eframe::egui;
use std::time::{Duration, Instant};

// todo: do the struct
pub struct Boom {
    start_time: Instant,
    playing: bool,
    weather: String,
    has_played: bool,
    flash_end_time: Option<Instant>,
    active_timer: Option<(Instant, Box<dyn FnOnce(&mut Self)>)>, // stores a code snippet
    camera_x: f32,
    moving_backwards: bool,
    clouds_enabled: bool,
}

impl Boom {
    //-----------------------------------------//
    // functions for the ui                    //
    //-----------------------------------------//
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            playing: false,
            weather: String::new(),
            has_played: false,
            flash_end_time: None,
            active_timer: None,
            camera_x: 0.0,
            moving_backwards: false,
            clouds_enabled: false,
        }
    }

    // func without pub are just helper functions
    pub fn trigger_thunder(&mut self) {
        // js make a white flash
        self.flash_end_time = Some(Instant::now() + Duration::from_millis(20));
        println!("thunder")
    }

    fn start_clouds(&self, ui: &mut egui::Ui) {
        let cloud_info = [
            (100.0, 80.0, 0.2),  // background (slow)
            (400.0, 120.0, 0.5), // midground (normal)
            (750.0, 60.0, 0.8),  // foreground (fast)
        ];

        let margin = 150.0;
        let rect = ui.max_rect();
        let screen_width = rect.width();
        let total_width = screen_width + (margin * 2.0);

        for (base_x, y_pos, depth_scale) in cloud_info {
            let raw_x = base_x + (self.camera_x * depth_scale);
            let wrapped_x = (raw_x % total_width) - margin;
            let final_x = rect.left() + wrapped_x;

            let center = egui::pos2(final_x, rect.top() + y_pos);
            let cloud_rect = egui::Rect::from_center_size(center, egui::vec2(225.0, 125.0));

            egui::Image::new(egui::include_image!("media/cloud.png"))
                .max_size(egui::vec2(225.0, 125.0))
                .paint_at(ui, cloud_rect);
        }
    }

    /* old cloud rendering code
    let painter = ui.painter();
    let rect = ui.max_rect();
    let screen_width = rect.width();

    let cloud_blueprints = [
        (100.0, 80.0, 0.2),  // background (slow)
        (400.0, 120.0, 0.5), // midground (normal)
        (750.0, 60.0, 0.8),  // foreground (fast)
    ];

    let margin = 100.0;
    let total_width = screen_width + (margin * 2.0);

    for (base_x, y_pos, depth_scale) in cloud_blueprints {
        let raw_x = base_x + (self.camera_x * depth_scale);

        let wrapped_x = (raw_x % total_width) - margin;
        let final_x = rect.left() + wrapped_x;

        let cloud_color = egui::Color32::from_white_alpha(200);

        let center1 = egui::pos2(final_x, rect.top() + y_pos);
        let center2 = egui::pos2(final_x + 30.0, rect.top() + y_pos + 10.0);
        let center3 = egui::pos2(final_x - 30.0, rect.top() + y_pos + 10.0);

        painter.circle_filled(center1, 40.0, cloud_color);
        painter.circle_filled(center2, 30.0, cloud_color);
        painter.circle_filled(center3, 30.0, cloud_color);
    }
    */

    fn parallax_effect(&self) {
        // make 2 quarter circles and put them on both sides mirrored over each other
        // make them overlap then duplicate to make it look like hills going backwards
        // add clouds also
    }

    fn start_screen(&self) {
        // stickman on a picnic on a flat grass land with clouds
        // transition backwards with cool tween into parallax_effect()

        self.parallax_effect();
    }

    fn change_weather(&mut self) {}

    // functions for waiting

    // bug: if one wait is already active, the old wait will be disposed and the new one will start
    pub fn wait(&mut self, seconds: f32, action: impl FnOnce(&mut Self) + 'static) {
        let target_time = Instant::now() + Duration::from_secs_f32(seconds);
        self.active_timer = Some((target_time, Box::new(action)));
    }

    pub fn update_background_timers(&mut self) {
        if let Some((target_time, _)) = self.active_timer {
            if Instant::now() >= target_time {
                if let Some((_, action)) = self.active_timer.take() {
                    action(self);
                }
            }
        }
    }

    //-----------------------------------------//
    // functions for the functionality (w.i.p) //
    //-----------------------------------------//

    fn play_audio(&mut self) {
        self.has_played = true;
        // Your audio playing code here
    }
}

impl eframe::App for Boom {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        self.update_background_timers();

        // thunder logic
        if let Some(end_time) = self.flash_end_time {
            if Instant::now() < end_time {
                ctx.request_repaint();

                egui::CentralPanel::default()
                    .frame(egui::Frame::none().fill(egui::Color32::WHITE))
                    .show(ctx, |_ui| {});

                return;
            } else {
                self.flash_end_time = None;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");

            if self.clouds_enabled == true {
                self.start_clouds(ui);
            }

            let dt = ctx.input(|i| i.stable_dt);
            self.camera_x += 75.0 * dt;

            if self.moving_backwards {
                self.camera_x -= 150.0 * dt;
            }

            if ui.button("Thunder!!").clicked() {
                self.wait(1.0, |state| {
                    state.trigger_thunder();
                });
            }

            if ui.button("Enable clouds").clicked() {
                self.clouds_enabled = true;
            }

            if ui.button("Move faster").clicked() {
                self.moving_backwards = true;

                self.wait(2.0, |state| {
                    state.moving_backwards = false;
                });
            }
        });
    }
}

pub fn init() -> eframe::Result<()> {
    eframe::run_native(
        "Boom Audio",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Boom::new()))
        }),
    )
}
