use crate::ant::parts::*;
use bevy::prelude::*;
use bevy_inspector_egui::egui::mutex::Mutex;
use bevy_prototype_lyon::prelude::tess::geom::Arc;

mod parts;

#[derive(Component)]
pub struct Ant {
    is_adult: bool,
    health: Stat,
    carb: Stat,
    prot: Stat,
    store: Option<Store>,
    parts: Vec<Box<dyn AntPart>>,
}

pub struct Store {
    carb: Stat,
    prot: Stat,
}

pub struct Stat {
    max: f32,
    cur: f32,
}
