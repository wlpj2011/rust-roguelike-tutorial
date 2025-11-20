use crate::Position;

use super::{Map, Monster, Name, Viewshed};
use rltk::{Point, console};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut monster_pos) = data;

        for (viewshed, _monster, name, monster_pos) in
            (&mut viewshed, &monster, &name, &mut monster_pos).join()
        {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("{} shouts insults.", name.name));
                let path = rltk::a_star_search(
                    map.xy_idx(monster_pos.x, monster_pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );
                if path.success && path.steps.len() > 1 {
                    monster_pos.x = path.steps[1] as i32 % map.width;
                    monster_pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
