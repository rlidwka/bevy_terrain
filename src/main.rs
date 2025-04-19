#![allow(clippy::too_many_arguments)]
mod orbit_camera;

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_terrain::terrain_common::{MeshStyle, Terrain, TerrainMeshResource};
use bevy_terrain::terrain_material::TerrainMaterial;
use bevy_terrain::terrain_rtin::{RtinParams, rtin_load_terrain};

use crate::orbit_camera::{OrbitCamera, OrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(OrbitCameraPlugin)
        .add_plugins(MaterialPlugin::<TerrainMaterial>::default())
        .register_type::<TerrainMeshResource>()
        .register_type::<MeshStyle>()
        .register_type::<RtinParams>()
        .init_resource::<TerrainMeshResource>()
        .init_resource::<MeshStyle>()
        .init_resource::<RtinParams>()
        .add_systems(Startup, setup)
        .add_systems(Update, ui)
        .add_observer(spawn_terrain)
        .run();
}

struct UiSettings {
    auto_generate: bool,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self { auto_generate: true }
    }
}

fn ui(
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
    mut mesh_style: ResMut<MeshStyle>,
    mut rtin_params: ResMut<RtinParams>,
    mut ui_settings: Local<UiSettings>,
) {
    let Some(ctx) = egui_contexts.try_ctx_mut() else { return };

    bevy_egui::egui::Window::new("Settings")
        .auto_sized()
        .max_width(250.)
        .show(ctx, |ui| {
            let mut redraw = false;
            let mut recalculate = false;

            ui.label("Max Height");

            recalculate |= ui.add(bevy_egui::egui::Slider::new(
                &mut rtin_params.load_options.max_image_height,
                0.0..=100.0,
            )).changed();

            ui.add_space(10.);

            ui.label("Error Threshold");

            recalculate |= ui.add(bevy_egui::egui::Slider::new(
                &mut rtin_params.error_threshold,
                0.0..=1.0,
            )).changed();

            ui.add_space(10.);

            ui.label("Mesh Style");

            bevy_egui::egui::ComboBox::from_id_salt("Mesh Style")
                .selected_text(format!("{:?}", &*mesh_style))
                .show_ui(ui, |ui| {
                    redraw |= ui.selectable_value(&mut *mesh_style, MeshStyle::Shaded, format!("{:?}", MeshStyle::Shaded)).changed();
                    redraw |= ui.selectable_value(&mut *mesh_style, MeshStyle::Wireframe, format!("{:?}", MeshStyle::Wireframe)).changed();
                });

            ui.add_space(10.);

            ui.horizontal(|ui| {
                if ui.button("generate").clicked() {
                    commands.trigger(SpawnTerrain { recalculate: true });
                }

                ui.checkbox(&mut ui_settings.auto_generate, "auto");
            });

            if ui_settings.auto_generate && recalculate || redraw {
                commands.trigger(SpawnTerrain { recalculate });
            }
        });
}

fn setup(mut commands: Commands) {
    // commands
    //     .spawn((
    //         DirectionalLight {
    //             illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         Transform {
    //             rotation: Quat::from_euler(EulerRot::XYZ, -0.6, 0.8, 0.),
    //             ..default()
    //         },
    //     ));

    let cam_transform = Transform::from_translation(Vec3::new(140., 70., 100.))
        .looking_at(Vec3::default(), Vec3::Y);
    let (yaw, pitch, _roll) = cam_transform.rotation.to_euler(EulerRot::YXZ);

    commands.spawn((
        Camera3d::default(),
        OrbitCamera {
            gimbal_x: -yaw,
            gimbal_y: -pitch,
            distance: cam_transform.translation.length(),
            ..default()
        },
        cam_transform,
        Msaa::Sample4,
    ));

    commands.trigger(SpawnTerrain { recalculate: true });
}

#[derive(Event)]
struct SpawnTerrain {
    recalculate: bool,
}

fn spawn_terrain(
    trigger: Trigger<SpawnTerrain>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    rtin_params: Res<RtinParams>,
    mut terrain_mesh_res: ResMut<TerrainMeshResource>,
    prev_terrain: Query<Entity, With<Terrain>>,
    mesh_style: Res<MeshStyle>,
) {
    for entity in prev_terrain.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if trigger.recalculate {
        let image_filename = "terrain.png";

        let (terrain_shaded_mesh, terrain_wireframe_mesh) =
            rtin_load_terrain(image_filename, &rtin_params);

        let terrain_shaded_mesh_handle = meshes.add(terrain_shaded_mesh);
        let terrain_wireframe_mesh_handle = meshes.add(terrain_wireframe_mesh);

        terrain_mesh_res.shaded = Mesh3d(terrain_shaded_mesh_handle);
        terrain_mesh_res.wireframe = Mesh3d(terrain_wireframe_mesh_handle);
    }

    let mesh = match *mesh_style {
        MeshStyle::Shaded => terrain_mesh_res.shaded.clone(),
        MeshStyle::Wireframe => terrain_mesh_res.wireframe.clone(),
    };

    commands.spawn((
        mesh,
        MeshMaterial3d(materials.add(TerrainMaterial {
            color: Color::WHITE.into(),
        })),
        Transform::default(),
        Terrain,
    ));
}
