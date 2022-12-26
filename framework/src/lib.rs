use bevy::{ecs::system::BoxedSystem, prelude::*};

pub trait Story
where
    Self: Send + Sync,
{
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn license(&self) -> String;
    fn run(&self) -> BoxedSystem;
}

pub trait Chapter
where
    Self: Send + Sync,
{
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn license(&self) -> String;
    fn stories(&self) -> Vec<Box<dyn Story>>;
}
