use super::{Map, Position, Rect, TileType, spawner};
mod simple_map;
use simple_map::SimpleMapBuilder;
mod common;
use common::*;
use specs::prelude::*;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&mut self) -> Map;
    fn get_starting_position(&mut self) -> Position;
}

pub fn random_builder(_new_depth: i32) -> Box<dyn MapBuilder> {
    Box::new(SimpleMapBuilder::new(_new_depth))
}
