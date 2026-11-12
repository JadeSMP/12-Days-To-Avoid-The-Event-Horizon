use bevy::prelude::*;
use bevy::sprite::Anchor;

const DAY_LENGTH_SECONDS: f32 = 11.0 * 60.0;
const PLAYER_SPEED: f32 = 220.0;
const PLAYER_HALF_SIZE: Vec2 = Vec2::splat(11.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<DayClock>()
        .init_resource::<DeveloperMode>()
        .init_resource::<GameMessage>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_developer_mode,
                advance_day_clock,
                move_player,
                interact_with_console,
                update_hud,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Solid {
    half_size: Vec2,
}

#[derive(Component)]
struct NavigationConsole;

#[derive(Component)]
struct MainHud;

#[derive(Component)]
struct MessageHud;

#[derive(Component)]
struct DeveloperHud;

#[derive(Resource)]
struct DayClock {
    day: u8,
    elapsed: f32,
    complete: bool,
}

impl Default for DayClock {
    fn default() -> Self {
        Self {
            day: 1,
            elapsed: 0.0,
            complete: false,
        }
    }
}

#[derive(Resource, Default)]
struct DeveloperMode(bool);

#[derive(Resource)]
struct GameMessage(String);

impl Default for GameMessage {
    fn default() -> Self {
        Self("Find the navigation console in the cockpit.".to_string())
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Ship deck.
    spawn_rect(
        &mut commands,
        Vec2::ZERO,
        Vec2::new(980.0, 600.0),
        Color::srgb(0.055, 0.075, 0.095),
        -10.0,
    );

    // Subtle deck panels.
    for x in (-400..=400).step_by(100) {
        spawn_rect(
            &mut commands,
            Vec2::new(x as f32, 0.0),
            Vec2::new(2.0, 560.0),
            Color::srgb(0.075, 0.10, 0.12),
            -5.0,
        );
    }

    // Outer hull walls.
    spawn_wall(&mut commands, Vec2::new(0.0, 289.0), Vec2::new(980.0, 22.0));
    spawn_wall(&mut commands, Vec2::new(0.0, -289.0), Vec2::new(980.0, 22.0));
    spawn_wall(&mut commands, Vec2::new(-489.0, 0.0), Vec2::new(22.0, 600.0));
    spawn_wall(&mut commands, Vec2::new(489.0, 0.0), Vec2::new(22.0, 600.0));

    // Room partitions. Gaps are intentional doorways.
    spawn_wall(&mut commands, Vec2::new(-120.0, 205.0), Vec2::new(700.0, 18.0));
    spawn_wall(&mut commands, Vec2::new(-120.0, -205.0), Vec2::new(700.0, 18.0));
    spawn_wall(&mut commands, Vec2::new(-120.0, 0.0), Vec2::new(18.0, 390.0));
    spawn_wall(&mut commands, Vec2::new(210.0, 105.0), Vec2::new(18.0, 200.0));
    spawn_wall(&mut commands, Vec2::new(210.0, -105.0), Vec2::new(18.0, 200.0));
    spawn_wall(&mut commands, Vec2::new(365.0, 0.0), Vec2::new(18.0, 190.0));

    // Room equipment and consoles.
    spawn_rect(
        &mut commands,
        Vec2::new(-330.0, 95.0),
        Vec2::new(92.0, 48.0),
        Color::srgb(0.13, 0.22, 0.28),
        1.0,
    );
    spawn_rect(
        &mut commands,
        Vec2::new(330.0, 110.0),
        Vec2::new(110.0, 55.0),
        Color::srgb(0.20, 0.16, 0.12),
        1.0,
    );
    spawn_rect(
        &mut commands,
        Vec2::new(330.0, -110.0),
        Vec2::new(110.0, 55.0),
        Color::srgb(0.12, 0.19, 0.20),
        1.0,
    );

    // Navigation console in the cockpit.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.10, 0.72, 0.78),
            custom_size: Some(Vec2::new(52.0, 34.0)),
            ..default()
        },
        Transform::from_xyz(355.0, 0.0, 3.0),
        NavigationConsole,
    ));

    // Player starts in the central corridor.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.92, 0.88, 0.72),
            custom_size: Some(PLAYER_HALF_SIZE * 2.0),
            ..default()
        },
        Transform::from_xyz(-30.0, 0.0, 5.0),
        Player,
    ));

    // Room labels.
    spawn_label(&mut commands, "ENGINEERING", Vec2::new(-390.0, 235.0), 2.0);
    spawn_label(&mut commands, "LIFE SUPPORT", Vec2::new(-390.0, -235.0), 2.0);
    spawn_label(&mut commands, "CREW QUARTERS", Vec2::new(45.0, 235.0), 2.0);
    spawn_label(&mut commands, "AIRLOCK", Vec2::new(310.0, 235.0), 2.0);
    spawn_label(&mut commands, "COCKPIT", Vec2::new(300.0, -235.0), 2.0);
    spawn_label(&mut commands, "NAV CONSOLE", Vec2::new(300.0, 42.0), 4.0);

    // Screen-space-style HUD text placed in the camera's visible area.
    commands.spawn((
        Text2d::new("DAY 1 / 12   |   11:00"),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.80, 0.94, 0.98)),
        Anchor::TOP_LEFT,
        Transform::from_xyz(-620.0, 340.0, 20.0),
        MainHud,
    ));
    commands.spawn((
        Text2d::new("Find the navigation console in the cockpit."),
        TextFont::from_font_size(16.0),
        TextColor(Color::srgb(0.72, 0.82, 0.86)),
        Anchor::TOP_LEFT,
        Transform::from_xyz(-620.0, 305.0, 20.0),
        MessageHud,
    ));
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(15.0),
        TextColor(Color::srgb(0.95, 0.75, 0.35)),
        Anchor::TOP_LEFT,
        Transform::from_xyz(-620.0, 265.0, 20.0),
        DeveloperHud,
    ));
}

fn spawn_rect(
    commands: &mut Commands,
    position: Vec2,
    size: Vec2,
    color: Color,
    z: f32,
) {
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, z),
    ));
}

fn spawn_wall(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.20, 0.27, 0.32),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 2.0),
        Solid {
            half_size: size / 2.0,
        },
    ));
}

fn spawn_label(commands: &mut Commands, label: &str, position: Vec2, z: f32) {
    commands.spawn((
        Text2d::new(label),
        TextFont::from_font_size(13.0),
        TextColor(Color::srgb(0.36, 0.50, 0.56)),
        Anchor::CENTER,
        Transform::from_xyz(position.x, position.y, z),
    ));
}

fn toggle_developer_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut developer_mode: ResMut<DeveloperMode>,
) {
    if keys.just_pressed(KeyCode::F1) {
        developer_mode.0 = !developer_mode.0;
    }
}

fn advance_day_clock(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    developer_mode: Res<DeveloperMode>,
    mut clock: ResMut<DayClock>,
) {
    if clock.complete {
        return;
    }

    if developer_mode.0 && keys.just_pressed(KeyCode::F2) {
        finish_current_day(&mut clock);
        return;
    }

    clock.elapsed += time.delta_secs();
    if clock.elapsed >= DAY_LENGTH_SECONDS {
        finish_current_day(&mut clock);
    }
}

fn finish_current_day(clock: &mut DayClock) {
    if clock.day >= 12 {
        clock.complete = true;
    } else {
        clock.day += 1;
        clock.elapsed = 0.0;
    }
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    solids: Query<(&Transform, &Solid), Without<Player>>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let movement = direction.normalize() * PLAYER_SPEED * time.delta_secs();

    // Resolve each axis independently so the player can slide along walls.
    transform.translation.x += movement.x;
    let player_position = transform.translation.truncate();
    for (wall_transform, wall) in &solids {
        if overlaps(player_position, PLAYER_HALF_SIZE, wall_transform.translation.truncate(), wall.half_size) {
            transform.translation.x -= movement.x;
            break;
        }
    }

    transform.translation.y += movement.y;
    let player_position = transform.translation.truncate();
    for (wall_transform, wall) in &solids {
        if overlaps(player_position, PLAYER_HALF_SIZE, wall_transform.translation.truncate(), wall.half_size) {
            transform.translation.y -= movement.y;
            break;
        }
    }
}

fn overlaps(a_pos: Vec2, a_half: Vec2, b_pos: Vec2, b_half: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < a_half.x + b_half.x
        && (a_pos.y - b_pos.y).abs() < a_half.y + b_half.y
}

fn interact_with_console(
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    console_query: Query<&Transform, With<NavigationConsole>>,
    mut message: ResMut<GameMessage>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    let (Ok(player), Ok(console)) = (player_query.single(), console_query.single()) else {
        return;
    };

    let distance = player
        .translation
        .truncate()
        .distance(console.translation.truncate());

    if distance <= 75.0 {
        message.0 = "NAV CONSOLE ONLINE — navigation controls are responding.".to_string();
    } else {
        message.0 = "Move closer to the navigation console to use it.".to_string();
    }
}

fn update_hud(
    clock: Res<DayClock>,
    developer_mode: Res<DeveloperMode>,
    message: Res<GameMessage>,
    mut main_hud: Query<&mut Text2d, (With<MainHud>, Without<MessageHud>, Without<DeveloperHud>)>,
    mut message_hud: Query<&mut Text2d, (With<MessageHud>, Without<MainHud>, Without<DeveloperHud>)>,
    mut developer_hud: Query<&mut Text2d, (With<DeveloperHud>, Without<MainHud>, Without<MessageHud>)>,
) {
    let remaining = (DAY_LENGTH_SECONDS - clock.elapsed).max(0.0);
    let minutes = (remaining / 60.0).floor() as u32;
    let seconds = (remaining % 60.0).floor() as u32;

    if let Ok(mut text) = main_hud.single_mut() {
        if clock.complete {
            text.0 = "JOURNEY COMPLETE — YOU SURVIVED ALL 12 DAYS".to_string();
        } else {
            text.0 = format!(
                "DAY {} / 12   |   {:02}:{:02} REMAINING",
                clock.day, minutes, seconds
            );
        }
    }

    if let Ok(mut text) = message_hud.single_mut() {
        text.0 = message.0.clone();
    }

    if let Ok(mut text) = developer_hud.single_mut() {
        text.0 = if developer_mode.0 {
            "DEVELOPER MODE  |  F1: close panel  |  F2: skip day".to_string()
        } else {
            "F1: developer mode".to_string()
        };
    }
}
