use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::Rng;
use rand::prelude::random;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
       // .insert_resource(PillarSpawnTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_system(fly)
        .add_system(fly_on_space)
       // .add_system(spawn_pillars)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(3.0))
                .with_system(spawn_pillar),
        )
        .add_system(move_pillar)
        .run();
}

// #[derive(Resource)]
// struct PillarSpawnTimer(Timer);

#[derive(Component)]
struct Player {
    gravity: f32,
    force: f32,
}

#[derive(Component)]
struct Pillar {
    height: f32,
    with: f32,
    direction: Direction,
    speed: f32,
}

enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::PURPLE,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform::from_xyz(-500.0, 0.0, 0.0),
        ..default()
    },
                    Player {
                        gravity: 200.0,
                        force: 0.0,
                    }
    ));
}

fn fly(time: Res<Time>, mut players: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in &mut players {
        transform.translation.y += (player.force - player.gravity) * time.delta_seconds();

        if player.force > 100.0 {
            player.force *= 0.5;
        } else {
            player.force = 0.0;
        }
    }
}

fn fly_on_space(keyboard_input: Res<Input<KeyCode>>, mut players: Query<&mut Player> ) {
    for mut player in &mut players {
        if keyboard_input.pressed(KeyCode::Space) {
            player.force += 1000.0;
        }
    }
}

fn spawn_pillar(mut commands: Commands) {
   // let mut rng = rand::thread_rng;

    let pillar = Pillar {
        height: random::<f32>() * 250.0,
        with: 30.0,
        direction: Direction::Up,
        speed: 100.0,
    };

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(pillar.with, pillar.height)),
            ..default()
        },
        transform: Transform::from_xyz(500.0, random::<f32>() * 500.0 - 250.0, 0.0),
        ..default()
    }, pillar
    ));
}

fn move_pillar(time: Res<Time>, mut pillars: Query<(&mut Pillar, &mut Transform)>) {
    for (mut pillar, mut transform) in &mut pillars {
        transform.translation.x -= pillar.speed * time.delta_seconds();
    }
}

