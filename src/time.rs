use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UniversalTime(pub f64);

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UniversalTime>()
            .add_systems(Update, tick_universal_time);
    }
}

fn tick_universal_time(time: Res<Time>, mut ut: ResMut<UniversalTime>) {
    ut.0 += time.delta_secs_f64();
}
