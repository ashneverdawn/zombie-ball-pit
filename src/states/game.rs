use amethyst::prelude::*;
use amethyst::{SimpleState, StateData, GameData};
use amethyst_imgui::imgui;
use amethyst::input;
use std::time::Instant;
use crate::entities::ball;
use crate::entities::player;

pub struct GameState{
    last_spawn_time : Instant,
    ball_count : i32
}
impl Default for GameState {
	fn default() -> Self {
        GameState {
            last_spawn_time : Instant::now(),
            ball_count : 0,
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.last_spawn_time = Instant::now();
        player::create(data.world);
    }
    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if input::is_close_requested(&event) || input::is_key_down(&event, input::VirtualKeyCode::Escape) {
                return Trans::Quit
            }
        }
        Trans::None
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.render_ui();

        if self.last_spawn_time.elapsed().as_millis() > 500 && self.ball_count < 15 {
            ball::create(data.world);
            self.last_spawn_time = Instant::now();
            self.ball_count += 1;
        }
        
        Trans::None
    }
}

impl GameState {
    fn render_ui(&mut self) {
        amethyst_imgui::with(|ui| {
            imgui::Window::new(imgui::im_str!("Window"))
                    .bg_alpha(0.35)
                    .no_decoration()
                    .always_auto_resize(true)
                    .save_settings(false)
                    .focus_on_appearing(false)
                    .no_nav()
                    .movable(false)
                    .build(ui, || {
                        ui.text(String::from("Game is running..."));
                    }
                ); 
        });
    }
}