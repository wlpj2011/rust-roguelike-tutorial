use super::MapBuilder;
use super::Map;
use specs::prelude::*;

pub struct SimpleMapBuilder {}

impl MapBuilder for SimpleMapBuilder {
    fn build(new_depth: i32) -> Map {
        Map::new(new_depth);
    }
}