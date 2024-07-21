use bevy::prelude::*;


#[derive(Component)]
pub struct TargetPosition {
    pub target: IVec2
}


impl TargetPosition {
    pub fn collide(&self, other: &TargetPosition) -> bool {
        if self.target == other.target {
            true
        } else {
            false
        }
    }
}