use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::prelude::random;

const PLAYER_SIZE: f32 = 50.0;
const GRAVITY: f32 = 200.0;
const PLAYER_START_POSITION : Vec3 = Vec3 { x: -500.0, y: 0.0, z: 0.0 };
const PILLAR_SPAWN_INTERVAL: f64 = 3.5;
const PLAYER_FORCE_ON_FLAP : f32 = 1000.0;
const PILLAR_WIDTH : f32 = 30.0;
const PILLAR_HEIGHT_SCALE_FACTOR: f32 = 250.0;
const PILLAR_DEFAULT_SPEED: f32 = 0.0;
const WORLD_DEFAULT_SPEED: f32 = 100.0;




fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
       // .insert_resource(PillarSpawnTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_system(fly)
        .add_system(fly_on_space)
        .add_system(check_for_collision)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(PILLAR_SPAWN_INTERVAL))
                .with_system(spawn_pillar),
        )
        .add_system(move_pillar)
        .run();
}

#[derive(Component)]
struct Player {
    gravity: f32,
    force: f32,
    width: f32,
    height: f32,
}

#[derive(Component)]
struct Pillar {
    height: f32,
    width: f32,
    speed: f32,
}

enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let player =  Player {
        gravity: GRAVITY,
        force: 0.0,
        width: PLAYER_SIZE,
        height: PLAYER_SIZE,
    };

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::PURPLE,
            custom_size: Some(Vec2::new(player.width, player.height)),
            ..default()
        },
        transform: Transform::from_translation(PLAYER_START_POSITION),
        ..default()
    }, player
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
            player.force += PLAYER_FORCE_ON_FLAP;
        }
    }
}

fn spawn_pillar(mut commands: Commands) {

    let pillar = Pillar {
        height: random::<f32>() * PILLAR_HEIGHT_SCALE_FACTOR,
        width: PILLAR_WIDTH,
        speed: PILLAR_DEFAULT_SPEED,
    };

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(pillar.width, pillar.height)),
            ..default()
        },
        transform: Transform::from_xyz(500.0, random::<f32>() * 500.0 - 250.0, 0.0),
        ..default()
    }, pillar
    ));
}

fn move_pillar(time: Res<Time>,
               mut pillars: Query<(&mut Pillar, &mut Transform)>,
) {
    for (pillar, mut transform) in &mut pillars {
        transform.translation.x -= (pillar.speed + WORLD_DEFAULT_SPEED) * time.delta_seconds();

    }
}

fn collide(player: &Player, player_transform: &Transform, pillar: &Pillar, pillar_transform: &Transform) -> bool {
    let rect1_left = player_transform.translation.x - ( player.width * 0.5 );
    let rect1_right = player_transform.translation.x + ( player.width * 0.5 );
    let rect1_top = player_transform.translation.y + ( player.height * 0.5 );
    let rect1_bottom = player_transform.translation.y - ( player.height * 0.5 );

    let rect2_left = pillar_transform.translation.x - ( pillar.width * 0.5 );
    let rect2_right = pillar_transform.translation.x + ( pillar.width * 0.5 );
    let rect2_top = pillar_transform.translation.y + ( pillar.width * 0.5 );
    let rect2_bottom = pillar_transform.translation.y - ( pillar.width * 0.5 );

    return rect1_left < rect2_right && rect1_right > rect2_left && rect1_top > rect2_bottom && rect1_bottom < rect2_top
}

fn check_for_collision(mut commands: Commands,
                        pillars: Query<( &Pillar, &Transform)>,
                        players: Query<( &Player, &Transform)>
                        ){

    for (player , player_transform) in players.iter() {
        for (pillar, pillar_transform) in pillars.iter() {
            if collide(player, player_transform, pillar,pillar_transform) {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(200.0, 100.0)),
                        ..default()
                    },
                    ..default()
                }
                );
            }
        }
    }
}




