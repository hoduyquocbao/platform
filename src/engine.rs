use crate::World;
use crate::resources::input::mod_rs::Resources;

pub trait System {
    fn run(&mut self, world: &mut World, resources: &mut Resources);
}
