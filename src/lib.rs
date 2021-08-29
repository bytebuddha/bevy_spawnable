use bevy_ecs::system::EntityCommands;
pub use bevy_spawnable_derive::Spawnable;

pub trait Spawnable {
    fn spawn(self, commands: &mut EntityCommands);
}
