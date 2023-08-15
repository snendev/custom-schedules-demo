use bevy::{
    ecs::schedule::ScheduleLabel,
    prelude::{
        shape, App, Assets, Camera3dBundle, Color, Commands, DefaultPlugins, IntoSystemConfigs,
        Mesh, PbrBundle, PointLight, PointLightBundle, ResMut, Resource, StandardMaterial, Startup,
        Transform, Update, Vec3, World,
    },
    window::close_on_esc,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_xpbd_3d::prelude::{Collider, PhysicsPlugins, PhysicsTimestep, Position, RigidBody};

#[derive(Clone, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
struct MyPhysicsSchedule;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin))
        .insert_resource(PhysicsTimestep::FixedOnce(1.0 / 60.0))
        .init_schedule(MyPhysicsSchedule)
        .init_resource::<SimulationTime>()
        .add_plugins(PhysicsPlugins::new(MyPhysicsSchedule))
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, (timeline_ui, execute_physics).chain())
        .run();
}

#[derive(Default, Resource)]
struct SimulationTime {
    current: u32,
    previous: u32,
}

fn timeline_ui(mut contexts: EguiContexts, mut simulation_time: ResMut<SimulationTime>) {
    simulation_time.previous = simulation_time.current;
    let ctx = contexts.ctx_mut();
    egui::Window::new("Window").show(ctx, |ui| {
        ui.label("Scrub the slider to progress the timeline.");
        let playforward_button = ui.button("Play forward");
        let playforward_hover_target = ui.button("Hover me!");
        if playforward_button.clicked() || playforward_hover_target.hovered() {
            simulation_time.current += 1;
        }
    });
}

fn execute_physics(world: &mut World) {
    let SimulationTime { current, previous } = world.resource::<SimulationTime>();
    for _ in *previous..*current {
        world.run_schedule(MyPhysicsSchedule);
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 6.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    // Plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(8.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 0.002, 8.0),
    ));

    let initial_position = Vec3::Y * 5.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::AQUAMARINE,
                ..Default::default()
            }),
            transform: Transform::from_translation(initial_position),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Position(initial_position),
        Collider::cuboid(1.0, 1.0, 1.0),
    ));
}
