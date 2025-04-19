use bevy::asset::RenderAssetUsages;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use image::{ImageBuffer, Luma};

use crate::terrain_common::TerrainImageLoadOptions;

pub fn terrain_example() -> Mesh {
    let options = TerrainImageLoadOptions {
        max_image_height: 1f32,
        pixel_side_length: 1f32,
    };

    let filename = "terrain.png";

    let mesh = load_terrain_bitmap(filename, options);
    mesh.unwrap()
}

fn sample_vertex_height(cy: i32, cx: i32, heightmap: &ImageBuffer<Luma<u16>, Vec<u16>>) -> f32 {
    let mut cnt = 0;
    let mut height = 0.0;

    for dy in [-1, 0].iter() {
        for dx in [-1, 0].iter() {
            let sy = cy + dy;
            let sx = cx + dx;
            if    sy < 0
               || sx < 0
               || sy >= heightmap.height() as i32
               || sx >= heightmap.width() as i32 {
                continue;
            } else {
                height += heightmap.get_pixel(sx as u32, sy as u32).0[0] as f32 * 1.0f32
                    / u16::MAX as f32;
                cnt += 1;
            }
        }
    }

    height / cnt as f32
}

fn load_terrain_bitmap(
    filename: &str,
    options: TerrainImageLoadOptions,
) -> Result<Mesh, image::ImageError> {
    let terrain_bitmap = image::open(filename)?;
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let heightmap = terrain_bitmap.as_luma16().unwrap();

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let vertex_number = ((heightmap.height() + 1) * (heightmap.width() + 1)) as usize;

    vertices.resize(vertex_number, [0.0f32, 0.0f32, 0.0f32]);
    let mut uvs = vec![[0.5, 0.5]; vertices.len()];

    let mut vertex_index = 0;
    for cy in 0..(heightmap.height() as i32 + 1) {
        for cx in 0..(heightmap.width() as i32 + 1) {
            let height = sample_vertex_height(cy, cx, heightmap);
            // println!("sampled height at y={:>3} x={:>3}  = {:>4}", cy, cx, height);

            vertices[vertex_index] = [
                cx as f32 * options.pixel_side_length,
                height * options.max_image_height,
                cy as f32 * options.pixel_side_length,
            ];
            uvs[vertex_index] = [
                cx as f32 / heightmap.width() as f32,
                cy as f32 / heightmap.height() as f32,
            ];

            vertex_index += 1;
        }
    }

    // let grid_height = heightmap.height() + 1;
    let grid_width = heightmap.width() + 1;

    for cy in 0..(heightmap.height()) {
        for cx in 0..(heightmap.width()) {
            indices.extend(
                [
                    cy * grid_width + cx,
                    (cy + 1) * grid_width + cx + 1,
                    cy * grid_width + cx + 1,
                ]
                .iter(),
            );
            indices.extend(
                [
                    cy * grid_width + cx,
                    (cy + 1) * grid_width + cx,
                    (cy + 1) * grid_width + cx + 1,
                ]
                .iter(),
            );
        }
    }

    // for i in 0..(indices.len()/3) {
    //     println!("triangle {:03}: {} {} {} ",
    //         i, indices[i*3], indices[i*3+1], indices[i*3+2])
    // }

    // println!(" {} {} ", indices.len() / 3, 2  * heightmap.height() * (heightmap.width()));

    assert!(indices.len() as u32 / 3 == 2 * heightmap.height() * (heightmap.width()));

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh.compute_smooth_normals();

    Ok(mesh)
}
