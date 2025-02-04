use bevy::prelude::*;

#[derive(Component)]
struct Name {
    name: String,
}

#[derive(Component)]
struct AnnualIncome {
    value: u32,
}

#[derive(Component)]
struct Human;

#[derive(Component)]
struct Monster;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_characters)
            .add_systems(PostStartup, hello_humans)
            .add_systems(Update, change_slime_size);
    }
}

fn spawn_characters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name {
            name: "Marisa".to_string(),
        },
        AnnualIncome { value: 30 },
        Human,
        Mesh3d(meshes.add(Sphere::new(3.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.5, 0.5))),
        Transform::from_xyz(5.0, 0.0, 0.0),
    ));
    commands.spawn((
        Name {
            name: "Slime".to_string(),
        },
        AnnualIncome { value: 12000 },
        Monster,
        Mesh3d(meshes.add(Sphere::new(3.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.5))),
        Transform::from_xyz(-5.0, 0.0, 0.0),
    ));
}

fn hello_humans(query: Query<(&Name, &AnnualIncome), With<Human>>) {
    for (name, annual_income) in query.iter() {
        println!(
            "こんにちは{}！年収は{}円ですね！",
            name.name, annual_income.value
        );
    }
}

fn change_slime_size(
    mut query: Query<&mut Transform, With<Monster>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let mut scale = transform.scale;
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            scale += Vec3::new(1.0, 1.0, 1.0) * time.delta_secs();
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            scale -= Vec3::new(1.0, 1.0, 1.0) * time.delta_secs();
        }

        *transform = transform.with_scale(scale);
    }
}
