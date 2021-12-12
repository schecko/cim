use bevy::{
    core::FixedTimestep,
    ecs::schedule::SystemSet,
    prelude::*,
    render::{camera::Camera, render_graph::base::camera::CAMERA_3D},
};
use rand::Rng;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

struct ScoreBoardText;
struct Controllable;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::MainMenu)

        .add_startup_system(setup_cameras.system())

        // main menu
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(display_mainmenu.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(enter_game.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu)
                .with_system(teardown.system())
        )

        // playing
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_active_unit.system())
                .with_system(focus_camera.system())
                .with_system(scoreboard_system.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing)
                .with_system(teardown.system())
        )

        // gameover
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(display_score.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(gameover_keyboard.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(teardown.system())
        )

        .run();
}

struct Cell {
    height: f32,
}

#[derive(Default)]
struct BoardUnit {
    i: usize,
    j: usize,
}

#[derive(Default)]
struct Settlement {
    handle: Handle<Scene>,
}

#[derive(Default)]
struct Game {
    board: Vec<Vec<Cell>>,
    active_unit: Option<Entity>,
    score: i32,
    cake_eaten: u32,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_I as f32 / 2.0,
    0.0,
    BOARD_SIZE_J as f32 / 2.0 - 0.5,
];

fn setup_cameras(mut commands: Commands, mut game: ResMut<Game>) {
    game.camera_should_focus = Vec3::from(RESET_FOCUS);
    game.camera_is_focus = game.camera_should_focus;
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(
            -(BOARD_SIZE_I as f32 / 2.0),
            2.0 * BOARD_SIZE_J as f32 / 3.0,
            BOARD_SIZE_J as f32 / 2.0 - 0.5,
        )
        .looking_at(game.camera_is_focus, Vec3::Y),
        ..Default::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    // reset the game state
    game.cake_eaten = 0;
    game.score = 0;

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..Default::default()
    });

    // spawn the game board
    let cell_scene = asset_server.load("models/AlienCake/tile.glb#Scene0");
    game.board = (0..BOARD_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I)
                .map(|i| {
                    let height = rand::thread_rng().gen_range(-0.1..0.1);
                    commands
                        .spawn_bundle((
                            Transform::from_xyz(i as f32, height - 0.2, j as f32) * Transform::from_scale(Vec3::new(1.0, 2.0, 1.0)),
                            GlobalTransform::identity(),
                        ))
                        .with_children(|cell| {
                            cell.spawn_scene(cell_scene.clone());
                        });
                    Cell { height }
                })
                .collect()
        })
        .collect();

    let settler_unit = BoardUnit{ i: BOARD_SIZE_I / 2, j: BOARD_SIZE_J / 2 };
    // spawn the initial settler
    game.active_unit = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: Vec3::new(
                        settler_unit.i as f32,
                        game.board[settler_unit.j][settler_unit.i].height,
                        settler_unit.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .with_children(|cell| {
                cell.spawn_scene(asset_server.load("models/AlienCake/alien.glb#Scene0"));
            })
            .insert(settler_unit)
            .insert(Controllable{})
            .id()
    );

    // load the scene for the cake
    //game.bonus.handle = asset_server.load("models/AlienCake/cakeBirthday.glb#Scene0");

    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Score:",
            TextStyle {
                font: asset_server.load("fonts/arialbd.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ScoreBoardText);
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// control the game character
fn move_active_unit(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut board_unit_transforms: Query<(&mut Transform, &mut BoardUnit), With<Controllable>>,
) {
    let active_unit = if let Some(id) = game.active_unit {
        id
    } else {
        return;
    };

    let (mut transform, mut board_unit) = if let Ok((transform, board_unit)) = board_unit_transforms.get_mut(active_unit) {
        (transform, board_unit)
    } else {
        game.active_unit = None;
        return;
    };

    let mut moved = false;
    let mut rotation = 0.0;
    if keyboard_input.just_pressed(KeyCode::Up) {
        if board_unit.i < BOARD_SIZE_I - 1 {
            board_unit.i += 1;
        }
        rotation = -std::f32::consts::FRAC_PI_2;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        if board_unit.i > 0 {
            board_unit.i -= 1;
        }
        rotation = std::f32::consts::FRAC_PI_2;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        if board_unit.j < BOARD_SIZE_J - 1 {
            board_unit.j += 1;
        }
        rotation = std::f32::consts::PI;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        if board_unit.j > 0 {
            board_unit.j -= 1;
        }
        rotation = 0.0;
        moved = true;
    }

    // move on the board
    if moved {
        transform.translation = Vec3::new(
            board_unit.i as f32,
            game.board[board_unit.j][board_unit.i].height,
            board_unit.j as f32,
        );
        transform.rotation = Quat::from_rotation_y(rotation);
        /*transform = Transform {
            translation: Vec3::new(
                board_unit.i as f32,
                game.board[board_unit.j][board_unit.i].height,
                board_unit.j as f32,
            ),
            rotation: Quat::from_rotation_y(rotation),
            ..Default::default()
        };*/
    }
}

// change the focus of the camera
fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
) {
    const SPEED: f32 = 2.0;
    // if there is both a player and a bonus, target the mid-point of them
    /*if let (Some(player_entity), Some(bonus_entity)) = (game.player.entity, game.bonus.entity) {
        if let (Ok(player_transform), Ok(bonus_transform)) = (
            transforms.q1().get(player_entity),
            transforms.q1().get(bonus_entity),
        ) {
            game.camera_should_focus = player_transform
                .translation
                .lerp(bonus_transform.translation, 0.5);
        }
    // otherwise, if there is only a player, target the player
    } else if let Some(player_entity) = game.player.entity {
        if let Ok(player_transform) = transforms.q1().get(player_entity) {
            game.camera_should_focus = player_transform.translation;
        }
    // otherwise, target the middle
    } else {
        game.camera_should_focus = Vec3::from(RESET_FOCUS);
    }
    // calculate the camera motion based on the difference between where the camera is looking
    // and where it should be looking; the greater the distance, the faster the motion;
    // smooth out the camera movement using the frame time
    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        game.camera_is_focus += camera_motion;
    }
    // look at that new camera's actual focus
    for (mut transform, camera) in transforms.q0_mut().iter_mut() {
        if camera.name == Some(CAMERA_3D.to_string()) {
            *transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
        }
    }*/
}

// update the score displayed during the game
fn scoreboard_system(game: Res<Game>, mut query: Query<&mut Text, With<ScoreBoardText>>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("Sugar Rush: {}", game.score);
}

fn gameover_keyboard(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}

fn enter_game(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}

// display the number of cake eaten before losing
fn display_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    format!("Cake eaten: {}", game.cake_eaten),
                    TextStyle {
                        font: asset_server.load("fonts/arialbd.ttf"),
                        font_size: 80.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

fn display_mainmenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Hello World",
                    TextStyle {
                        font: asset_server.load("fonts/arialbd.ttf"),
                        font_size: 80.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}
