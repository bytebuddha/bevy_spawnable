use bevy_ecs::prelude::*;
use bevy_ecs::system::EntityCommands;
pub use bevy_spawnable_derive::Spawnable;

pub trait Spawnable {
    fn spawn(self, commands: &mut EntityCommands);
}

impl <T>Spawnable for T
where
    T: Bundle
{
    fn spawn(self, commands: &mut EntityCommands) {
        commands.insert_bundle(self);
    }
}
