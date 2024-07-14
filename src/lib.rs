pub use bevy::prelude::*;
use bevy::{render::camera::RenderTarget, window::WindowRef};

pub struct PopOutPlugin;
impl Plugin for PopOutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PopOutEvent>()
            .add_event::<DespawnWindowEvent>()
            .add_systems(Update, (
                    spawn_pop_out_window.run_if(on_event::<PopOutEvent>()), 
                    despawn_pop_out_window.run_if(on_event::<DespawnWindowEvent>())
                ));
    }
}

#[derive(Event)]
pub struct PopOutEvent{
    pub add: Entity,
    pub pos: Transform,
    pub title: String,
}

impl PopOutEvent{
    pub fn new(add: Entity){
        Self{
            add,
            pos: Transform::default(),
            title: "".into(),
        }
    }

    pub fn with_pos(mut self, pos: Transform) -> Self{
        self.pos = pos;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

#[derive(Component)]
struct PopOutWindow;

#[derive(Component)]
struct PopOutCamera;

fn spawn_pop_out_window(
    mut commands: Commands,
    mut events: EventReader<PopOutEvent>,
){
    for event in events.read(){
        let PopOutEvent { add, pos, title } = event;
        let window_id = commands
            .spawn((
                Window {
                    title,
                    ..default()
                },
                PopOutCamera,
            )).id();

        let camera_id = commands
            .spawn((
                Camera2dBundle {
                    transform: pos,
                    camera: Camera {
                        target: RenderTarget::Window(WindowRef::Entity(window_id)),
                        ..default()
                    },
                    ..default()
                },
                PopOutCamera,
            )).id();

        let ui_root = commands
            .spawn((
                NodeBundle{
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                TargetCamera(camera_id),
            )).id();
        commands.entity(ui_root).add_child(*add);
    }
}

#[derive(Event)]
pub struct DespawnWindowEvent(pub Entity);

fn despawn_pop_out_window(
    mut events: EventReader<DespawnWindowEvent>,
    mut commands: Commands,
) {
    for DespawnWindowEvent(window_id) in events.read(){
        commands.entity(*window_id).despawn_recursive();
    }
}
