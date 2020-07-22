use amethyst::{
    prelude::*, 
    SimpleState, StateData, GameData,
    assets::{ProgressCounter, Completion, Handle},
    input,
    renderer::Mesh,
    core::math::{Point3},
};
use amethyst_imgui::imgui;
use crate::states::game;
use crate::entities::scene;
use crate::entities::arena;

pub struct LoadingState{
    progress_counter: Option<ProgressCounter>,
    mesh_data:      Option<(Handle<Mesh>, Vec<Point3<f32>>, Vec<Point3<usize>>)>,
}
impl Default for LoadingState {
	fn default() -> Self {
        LoadingState {
            progress_counter: Some(ProgressCounter::default()),
            mesh_data : None,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) { 
        scene::init(data.world, self.progress_counter.as_mut().unwrap());
        self.mesh_data = Some(arena::init(data.world, self.progress_counter.as_mut().unwrap()));
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
        
        if let Some(progress) = &self.progress_counter {
            match progress.complete() {
                Completion::Complete => {
                    scene::create(data.world);
                    arena::create(data.world, self.mesh_data.take().unwrap());
                    println!("Starting...");
                    return Trans::Switch(Box::new(game::GameState::default()));
                },
                Completion::Loading => { println!("Loading...") },
                Completion::Failed => { println!("Loading Failed!") },
            }
        }

        Trans::None
    }
}

impl LoadingState {
    fn render_ui(&mut self) {
        amethyst_imgui::with(|ui| {
            imgui::Window::new(imgui::im_str!("LoadingWindow"))
                    .bg_alpha(0.35)
                    .no_decoration()
                    .always_auto_resize(true)
                    .save_settings(false)
                    .focus_on_appearing(false)
                    .no_nav()
                    .movable(false)
                    .build(ui, || {
                        ui.text(String::from("Loading..."));
                    }
                ); 
        });
    }
}