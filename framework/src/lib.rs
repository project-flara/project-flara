use bevy::prelude::*;

pub trait Story {
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn license(&self) -> String;
    fn run(&self) -> SystemStage;
}

pub trait Chapter {
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn license(&self) -> String;
    fn stories(&self) -> Vec<Box<dyn Story>>;
}
