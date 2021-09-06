use bevy::prelude::*;
use bevy::ecs::world::EntityMut;
pub use bevy_spawnable_derive::Spawnable;

pub trait Spawnable: FromWorld {
    fn spawn(self, commands: EntityMut);
}

impl <T>Spawnable for T
where
    T: Bundle + Default
{
    fn spawn(self, mut commands: EntityMut) {
        commands.insert_bundle(self);
    }
}

pub fn spawn<S: Spawnable>(world: &mut World) {
    let spawnable = S::from_world(world);
    spawnable.spawn(world.spawn());
}
