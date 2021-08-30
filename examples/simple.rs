use bevy::prelude::*;
use bevy_devtools::{ DevToolsExt, DevToolsPlugin };
use bevy_spawnable::Spawnable;

#[derive(Spawnable, Default)]
pub struct ComponentHierarchy {
    pub name: Name,
    #[bundle]
    pub node: NodeBundle,
    #[child]
    pub component_a: ComponentA,
    #[child]
    pub component_b: ComponentB
}

#[derive(Spawnable)]
pub struct ComponentA {
    pub name: Name,
    #[bundle]
    pub node: NodeBundle,
    #[child]
    pub child: Child
}

impl Default for ComponentA {
    fn default() -> ComponentA {
        ComponentA {
            name: Name::new("Component A"),
            node: Default::default(),
            child: Default::default(),
        }
    }
}

#[derive(Spawnable)]
pub struct ComponentB {
    pub name: Name,
    #[bundle]
    pub node: NodeBundle,
    #[child]
    pub child_one: Child,
    #[child]
    pub child_two: Child
}

impl Default for ComponentB {
    fn default() -> ComponentB {
        ComponentB {
            name: Name::new("Component B"),
            node: Default::default(),
            child_one: Default::default(),
            child_two: Default::default(),
        }
    }
}

#[derive(Spawnable)]
pub struct Child {
    pub name: Name
}

impl Default for Child {
    fn default() -> Child {
        Child { name: "Child".into() }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DevToolsPlugin)
        .devtools_enabled()
        .add_startup_system(spawn_entities.system())
        .run()
}

fn spawn_entities(mut commands: Commands) {
    let entities = ComponentHierarchy {
        name: Name::new("Component Hierarchy"),
        ..Default::default()
    };

    entities.spawn(&mut commands.spawn());
}
