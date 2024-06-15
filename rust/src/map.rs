use godot::classes::*;
use godot::prelude::*;

use crate::camera::Camera;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Map {
    camera: OnReady<Gd<Camera>>,
    tile_map: OnReady<Gd<TileMap>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Map {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            camera: OnReady::manual(),
            tile_map: OnReady::manual(),
            base,
        }
    }

    fn ready(&mut self) {
        self.tile_map
            .init(self.base().get_node_as::<TileMap>("TileMap"));
        self.camera
            .init(self.base().get_node_as::<Camera>("Camera"));
        let map_limit = self.tile_map.get_used_rect();
        let tile_size = self.tile_map.get_tileset().unwrap().get_tile_size();
        self.camera
            .set_limit(Side::LEFT, map_limit.position.x * tile_size.x);
        self.camera
            .set_limit(Side::TOP, map_limit.position.y * tile_size.y);
        self.camera
            .set_limit(Side::RIGHT, map_limit.end().x * tile_size.x);
        self.camera
            .set_limit(Side::BOTTOM, map_limit.end().y * tile_size.y);
    }

    fn physics_process(&mut self, _delta: f64) {}
}
