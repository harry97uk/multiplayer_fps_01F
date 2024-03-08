use bevy::prelude::{ StandardMaterial, Handle };

pub struct Materials {
    pub player_material: Handle<StandardMaterial>,
    pub floor_material: Handle<StandardMaterial>,
    pub wall_material: Handle<StandardMaterial>,
}
