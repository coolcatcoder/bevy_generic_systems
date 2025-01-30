use bevy::prelude::*;
use bevy_generic_systems::prelude::*;
use bevy_registration::prelude::*;

mod timer;
use timer::Timer;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RegistrationPlugin))
        .run();
}

/// Allows you to know when your cake is done!
#[derive(Component, Behaviour)]
struct CakeTimer {
    timer: Timer,
    fun: f32,
}

/// Check to see if any cakes are done cooking.
#[system(Update)]
fn cake_timer(cake_timers: Query<&CakeTimer>) {
    cake_timers.iter().for_each(|cake_timer| {
        if cake_timer.timer.remaining == 0. {
            info!("Ding-a-ling-ling! Cake is done!")
        }
    });
}

/// Spawn a CakeTimer to show that it works.
#[system(Startup)]
fn setup(mut commands: Commands) {
    commands.spawn(CakeTimer {
        timer: Timer { remaining: 3. },
        fun: 0.,
    });
}
