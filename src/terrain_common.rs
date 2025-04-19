use bevy::prelude::*;

#[derive(Resource, Reflect, Debug, Eq, PartialEq, Default)]
#[reflect(Resource)]
pub enum MeshStyle {
    Shaded,
    #[default]
    Wireframe,
}

#[derive(Component)]
pub struct Terrain;

#[derive(Reflect)]
pub struct TerrainImageLoadOptions {
    pub max_image_height: f32,
    pub pixel_side_length: f32,
}

impl Default for TerrainImageLoadOptions {
    fn default() -> Self {
        Self {
            max_image_height: 20.0,
            pixel_side_length: 1.0,
        }
    }
}

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct TerrainMeshResource {
    pub shaded: Mesh3d,
    pub wireframe: Mesh3d,
}
