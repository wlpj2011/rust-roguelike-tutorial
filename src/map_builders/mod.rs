use super::Map;

trait MapBuilder {
    fn build(new_depth: i32) -> Map;
}