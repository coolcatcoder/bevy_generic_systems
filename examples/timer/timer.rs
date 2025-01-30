use bevy::prelude::*;
use bevy_generic_systems::prelude::*;

/// A simple timer. Once remaining is 0 then it is done!
// TODO: Allow schedule setting. Perhaps via generics?
pub struct Timer {
    pub remaining: f32,
}

/// Counts down timers.
fn timer_tick<T: ComponentContains<Timer>>(mut timers: Query<&mut T>, time: Res<Time>) {
    timers.iter_mut().for_each(|mut timer| {
        timer.get_mut().for_each(|timer| {
            timer.remaining -= time.delta_secs();
            timer.remaining = timer.remaining.max(0.);
        });
    });
}

// Define the behavour of Timer when it is a field.
impl ComponentFieldBehaviour for Timer {
    fn app<T: ComponentContains<Self>>(app: &mut App) {
        app.add_systems(Update, timer_tick::<T>);
    }
}
