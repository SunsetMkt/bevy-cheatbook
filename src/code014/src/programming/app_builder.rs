use bevy::prelude::*;

#[derive(Resource, Default)]
struct MyFancyResource;

#[derive(Resource, Default)]
struct StartingLevel(u32);

#[derive(Event)]
struct LevelUpEvent;

#[derive(Bundle, Default)]
struct SomeBundle {
    spatial: SpatialBundle,
}

fn spawn_things() {}
fn debug_stats_change() {}
fn camera_follow_player() {}
fn debug_levelups() {}

mod ui {
    use bevy::prelude::*;
    pub struct MyUiPlugin;
    impl Plugin for MyUiPlugin {
        fn build(&self, _: &mut App) {}
    }
}

// ANCHOR: appexit
use bevy::app::AppExit;

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}
// ANCHOR_END: appexit

// ANCHOR: main
fn main() {
    App::new()
        // Bevy itself:
        .add_plugins(DefaultPlugins)

        // Plugins from our game/project:
        .add_plugins(ui::MyUiPlugin)

        // events:
        .add_event::<LevelUpEvent>()

        // systems to run once at startup:
        .add_systems(Startup, spawn_things)

        // systems to run each frame:
        .add_systems(Update, (
            camera_follow_player,
            debug_levelups,
            debug_stats_change,
        ))
        // ...

        // launch the app!
        .run();
}
// ANCHOR_END: main

fn main2() {
    let mut app = App::new();

// ANCHOR: world
// Create (or overwrite) resource with specific value
app.insert_resource(StartingLevel(3));

// Ensure resource exists; if not, create it
// (using `Default` or `FromWorld`)
app.init_resource::<MyFancyResource>();

// We can also access/manipulate the World directly
// (in this example, to spawn an entity, but you can do anything)
app.world_mut().spawn(SomeBundle::default());
// ANCHOR_END: world
}
