use bevy::prelude::*;
use bevy_devtools::{ DevToolsExt, DevToolsPlugin };
use bevy_spawnable::{spawn, Spawnable};

#[derive(Spawnable)]
pub struct ComponentHierarchy {
    pub name: Name,
    #[child]
    pub component_one: ComponentA,
    #[child]
    pub component_two: ComponentB
}

impl FromWorld for ComponentHierarchy {
    fn from_world(world: &mut World) -> ComponentHierarchy {
        ComponentHierarchy {
            name: Name::new("Component Hierarchy"),
            component_one: FromWorld::from_world(world),
            component_two: FromWorld::from_world(world)
        }
    }
}

#[derive(Spawnable)]
pub struct ComponentA {
    pub name: Name,
    #[child]
    pub child: Child
}

impl FromWorld for ComponentA {
    fn from_world(world: &mut World) -> ComponentA {
        ComponentA {
            name: Name::new("Component A"),
            child: FromWorld::from_world(world)
        }
    }
}

#[derive(Spawnable)]
pub struct ComponentB {
    pub name: Name,
    #[child]
    pub child_one: Child,
    #[child]
    pub child_two: Child
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> ComponentB {
        ComponentB {
            name: Name::new("Component B"),
            child_one: FromWorld::from_world(world),
            child_two: FromWorld::from_world(world)
        }
    }
}


#[derive(Spawnable)]
pub struct Child {
    pub name: Name
}

impl FromWorld for Child {
    fn from_world(_: &mut World) -> Child {
        Child { name: Name::new("Child") }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DevToolsPlugin)
        .devtools_enabled()
        .add_startup_system(spawn::<ComponentHierarchy>.exclusive_system())
        .run()
}
