use common::comp::Ori;
use specs::Component;
use specs_idvs::IdvStorage;
use vek::*;

// Floats over entity that has had a health change, rising up over time until it
// vanishes
#[derive(Copy, Clone, Debug)]
pub struct HpFloater {
    pub timer: f32,
    // Numbers of times significant damage has been dealt
    pub hp_change: f32,
    // Used for randomly offsetting
    pub rand: f32,
}
#[derive(Clone, Debug, Default)]
pub struct HpFloaterList {
    // Order oldest to newest
    pub floaters: Vec<HpFloater>,
    // Keep from spawning more floaters from same hp change
    // Note: this can't detect a change if equivalent healing and damage take place simultaneously
    pub last_hp: f32,
    // The time since you last damaged this entity
    // Used to display nametags outside normal range if this time is below a certain value
    pub time_since_last_dmg_by_me: Option<f32>,
}
impl Component for HpFloaterList {
    type Storage = IdvStorage<Self>;
}

// Used for smooth interpolation of visual elements that are tied to entity
// position
#[derive(Copy, Clone, Debug)]
pub struct Interpolated {
    pub pos: Vec3<f32>,
    pub ori: Ori,
}
impl Component for Interpolated {
    type Storage = IdvStorage<Self>;
}
