use bevy::prelude::*;
use strum_macros::EnumIter;

#[derive(Resource, Default)]
pub struct UniversalTime(pub f64);

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UniversalTime>()
            .init_resource::<WarpFactor>()
            .add_systems(Update, tick_universal_time);
    }
}

fn tick_universal_time(
    time: Res<Time>,
    mut ut: ResMut<UniversalTime>,
    warp_factor: Res<WarpFactor>,
) {
    let delta = time.delta_secs_f64() * warp_factor.value();
    if delta != 0.0 {
        ut.0 += delta;
    }
}

#[derive(Resource, Default, PartialEq, Clone, Copy, EnumIter, Debug)]
pub enum WarpFactor {
    Paused,
    Half,
    #[default]
    One,
    Two,
    Three,
    Five,
    Ten,
    Hundred,
    Thousand,
    TenThousand,
    HundredThousand,
}
impl WarpFactor {
    fn value(&self) -> f64 {
        match self {
            Self::Paused => 0.0,
            Self::Half => 0.5,
            Self::One => 1.0,
            Self::Two => 2.0,
            Self::Three => 3.0,
            Self::Five => 5.0,
            Self::Ten => 10.0,
            Self::Hundred => 100.0,
            Self::Thousand => 1000.0,
            Self::TenThousand => 10000.0,
            Self::HundredThousand => 100000.0,
        }
    }
}
