use bevy::{prelude::*, math::vec2};

const MAX_FIELDS: usize = 5;
const SCR_SPRITE_SIZE: f32 = 16.0;
const SPRITE_SIZE: f32 = 96.0;
const SPRITE_SIZE_HALVED: f32 = SPRITE_SIZE / 2.0;
const W_WIDTH: f32 = 640.0;
const BATTLEFIELD_OFFSET: f32 = -200.0;
const W_HEIGHT: f32 = 480.0;

const HEROES_SPRITE_SHEET: &'static str = "heroes.png";
const UNDER_CONSTRUCTION_SPRITE: &'static str = "under-construction.png";
const FONT_SIZE: f32 = 8.0;

const NORMAL_BUTTON: Color = Color::rgba(0.0, 0.0, 0.0, 0.2);
const HOVERED_BUTTON: Color = Color::rgba(0.35, 0.35, 0.75, 0.5);
const PRESSED_BUTTON: Color = Color::rgba(0.35, 0.75, 0.35, 0.5);


type AnimRectSource = (f32, f32, f32, f32);


#[derive(Resource)]
struct WorldMousePos {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct ActionButton;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct FieldPosition(usize);

#[derive(Component)]
struct Health(isize);

#[derive(Resource)]
struct Battlefield {
    fields: [Option<Entity>; 5]
}

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Animation {
    frames: Vec<AnimRectSource>,
    repeat: bool,
    act_frame_index: usize
}

impl Animation {
    fn update(&mut self) {
        let mut next_frame_index = self.act_frame_index + 1;
        if next_frame_index >= self.frames.len() {
            if self.repeat {
                next_frame_index = 0;
            } else {
                next_frame_index = self.frames.len() - 1;
            }
        }
        self.act_frame_index = next_frame_index;
    }

    fn get_rect(&self) -> Rect {
        let (x, y, w, h) = self.frames[self.act_frame_index];
        Rect::new(x, y, x + w, y + h)
    }
}

struct ActorMovedEvent {
    actor: Entity,
    old_pos: usize,
    new_pos: usize,
}

struct ActorChangeFacingEvent {
    actor: Entity,
}

struct DamageActionEvent {
    target_pos: usize,
    damage: isize,
}

struct SpawnEnemyEvent {
    pos: usize,
    name: String,
    flip_x: bool,
}

struct ClearActorEvent {
    actor: Entity
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "MG-2".into(),
                    width: W_WIDTH,
                    height: W_HEIGHT,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    ..default()
                },
                ..default()
            })
        )
        .insert_resource(WorldMousePos { x: 0.0, y: 0.0 })
        .insert_resource(Battlefield { fields: [None; MAX_FIELDS] })
        .add_event::<ActorMovedEvent>()
        .add_event::<SpawnEnemyEvent>()
        .add_event::<ActorChangeFacingEvent>()
        .add_event::<DamageActionEvent>()
        .add_event::<ClearActorEvent>()
        .add_startup_system(setup)
        .add_system(spawn_enemy_event)
        .add_system(mouse_screen_to_world)
        .add_system(battlefield_button_click)
        .add_system(action_button_click)
        .add_system(actor_move_event)
        .add_system(actor_change_facing_event)
        .add_system(damage_action_event)
        .add_system(battlefield_button_states)
        .add_system(update_actor_debug_text)
        .add_system(clear_actor_event)
        .add_system(update_animation)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev: EventWriter<SpawnEnemyEvent>,
) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("QuinqueFive.ttf");
    let text_style = TextStyle {
        font,
        font_size: FONT_SIZE,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER_RIGHT;
    let player_pos = 0;

    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load(HEROES_SPRITE_SHEET),
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_SIZE, SPRITE_SIZE)),
                    rect: Some(Rect::new(4.0 * SCR_SPRITE_SIZE, 0.0, 5.0 * SCR_SPRITE_SIZE, SCR_SPRITE_SIZE)),
                    flip_x: false,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(BATTLEFIELD_OFFSET + player_pos as f32 * (SPRITE_SIZE + 2.0), 0.0, 100.0),
                    ..default()
                },
                ..default()
            },
            Player {},
            Health(5),
            Name("Hero".into()),
            FieldPosition(player_pos)
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section("", text_style.clone())
                        .with_alignment(text_alignment),
                    transform: Transform::from_xyz(
                        0.0,
                        SPRITE_SIZE_HALVED + FONT_SIZE * 2.0,
                        2.0,
                    ),
                    ..default()
                },
            ));
        });

    ev.send(SpawnEnemyEvent { pos: 3, name: "Orc".into(), flip_x: true });

    let top = (W_HEIGHT - SPRITE_SIZE) / 2.0;
    for i in 0..MAX_FIELDS {
        let left = 70.0 + i as f32 * (SPRITE_SIZE + 2.0);
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(SPRITE_SIZE), Val::Px(SPRITE_SIZE)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(left),
                            top: Val::Px(top),
                            ..default()
                        },
                        ..default()
                    },
                    // z_index: ZIndex::Global(-10),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                FieldPosition(i)
            ));
    }

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(SPRITE_SIZE), Val::Px(SPRITE_SIZE)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(16.0),
                        top: Val::Px(16.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                // z_index: ZIndex::Global(-10),
                ..default()
            },
            ActionButton
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: asset_server.load(UNDER_CONSTRUCTION_SPRITE).into(),
                z_index: ZIndex::Global(-10),
                ..default()
            });
        });
}

fn spawn_enemy_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut battlefield: ResMut<Battlefield>,
    mut ev: EventReader<SpawnEnemyEvent>,
) {
    let font = asset_server.load("QuinqueFive.ttf");
    let text_style = TextStyle {
        font,
        font_size: FONT_SIZE,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER_RIGHT;

    for e in ev.iter() {
        if battlefield.fields[e.pos].is_some() {
            eprintln!("Field {} is already occupied by {:?}", e.pos, battlefield.fields[e.pos].unwrap());
            continue;
        }

        let entity_id = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load(HEROES_SPRITE_SHEET),
                    sprite: Sprite {
                        custom_size: Some(vec2(SPRITE_SIZE, SPRITE_SIZE)),
                        rect: Some(Rect::new(3.0 * SCR_SPRITE_SIZE, 2.0 * SCR_SPRITE_SIZE, 4.0 * SCR_SPRITE_SIZE, 3.0 * SCR_SPRITE_SIZE)),
                        flip_x: e.flip_x,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(BATTLEFIELD_OFFSET + e.pos as f32 * (SPRITE_SIZE + 2.0), 0.0, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Health(5),
                Name(e.name.clone()),
                FieldPosition(e.pos)
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section("", text_style.clone())
                            .with_alignment(text_alignment),
                        transform: Transform::from_xyz(
                            0.0,
                            SPRITE_SIZE_HALVED + FONT_SIZE * 2.0,
                            2.0,
                        ),
                        ..default()
                    },
                ));
            })
            .id();

        battlefield.fields[e.pos] = Some(entity_id);
    }
}

fn update_actor_debug_text(
    q: Query<(&Name, &Health, &FieldPosition), Or<(Changed<Health>, Changed<FieldPosition>)>>,
    mut q_children: Query<(&mut Text, &Parent)>
) {
    for (mut text, parent) in q_children.iter_mut() {
        let Ok((name, health, field_pos)) = q.get(parent.get()) else {
            continue;
        };
        text.sections[0].value = format!("{}\n{}hp\n@{}", name.0, health.0, field_pos.0);
    }
}

fn update_animation(
    time: Res<Time>,
    mut q: Query<(&mut AnimationTimer, &mut Animation, &mut Sprite)>
) {
    for (mut anim_timer, mut animation, mut sprite) in q.iter_mut() {
        if anim_timer.0.tick(time.delta()).just_finished() {
            animation.update();
            sprite.rect = Some(animation.get_rect());
        }
    }
}

fn clear_actor_event(
    mut commands: Commands,
    mut battlefield: ResMut<Battlefield>,
    q: Query<&FieldPosition, With<Health>>,
    mut ev: EventReader<ClearActorEvent>
) {
    for e in ev.iter() {
        let Ok(field_pos) = q.get(e.actor) else {
            continue;
        };
        battlefield.fields[field_pos.0] = None;
        commands.entity(e.actor).despawn_recursive();
    }
}


fn actor_move_event(
    mut q: Query<(&mut FieldPosition, &mut Transform)>,
    mut battlefield: ResMut<Battlefield>,
    mut ev: EventReader<ActorMovedEvent>,
) {
    for e in ev.iter() {
        if battlefield.fields[e.new_pos].is_some() {
            continue;
        }
        let Ok((mut field_pos, mut transform)) = q.get_mut(e.actor) else {
            continue;
        };
        battlefield.fields[e.old_pos] = None;
        battlefield.fields[e.new_pos] = Some(e.actor);
        field_pos.0 = e.new_pos;
        transform.translation.x = BATTLEFIELD_OFFSET + e.new_pos as f32 * (SPRITE_SIZE + 2.0);
    }
}

fn actor_change_facing_event(
    mut q: Query<&mut Sprite>,
    mut ev: EventReader<ActorChangeFacingEvent>,
) {
    for e in ev.iter() {
        let Ok(mut sprite) = q.get_mut(e.actor) else {
            continue;
        };
        sprite.flip_x = !sprite.flip_x;
    }
}

fn damage_action_event(
    mut q: Query<(Entity, &mut Health)>,
    battlefield: Res<Battlefield>,
    mut ev: EventReader<DamageActionEvent>,
    mut ev_clear: EventWriter<ClearActorEvent>,
) {
    for e in ev.iter() {
        let Some(target_entity) = battlefield.fields[e.target_pos] else {
            continue;
        };
        let Ok((entity, mut health)) = q.get_mut(target_entity) else {
            continue;
        };
        health.0 -= e.damage;
        if health.0 <= 0 {
            ev_clear.send(ClearActorEvent { actor: entity });
        }
    }
}

fn mouse_screen_to_world(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut world_mouse_pos: ResMut<WorldMousePos>
) {
    let (camera, camera_transform) = q_camera.single();
    let Some(wnd) = wnds.get_primary() else {
        return
    };
    let Some(screen_pos) = wnd.cursor_position() else {
        return
    };

    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    world_mouse_pos.x = world_pos.x;
    world_mouse_pos.y = world_pos.y;
}

fn battlefield_button_click(
    mut interaction_query: Query<
        (&Interaction, &FieldPosition),
        (Changed<Interaction>, With<Button>),
    >,
    q: Query<(Entity, &FieldPosition), With<Player>>,
    mut ev_moved: EventWriter<ActorMovedEvent>,
    mut ev_facing: EventWriter<ActorChangeFacingEvent>,
) {
    for (interaction, clicked_field_pos) in &mut interaction_query {
        let Interaction::Clicked = *interaction else {
            continue;
        };
        let Ok((entity_id, field_pos)) = q.get_single() else {
            return;
        };
        if clicked_field_pos.0 == field_pos.0 {
            ev_facing.send(ActorChangeFacingEvent { actor: entity_id });
        }
        if clicked_field_pos.0 > field_pos.0 {
            ev_moved.send(ActorMovedEvent { actor: entity_id, old_pos: field_pos.0, new_pos: field_pos.0 + 1 });
        }
        if clicked_field_pos.0 < field_pos.0 {
            ev_moved.send(ActorMovedEvent { actor: entity_id, old_pos: field_pos.0, new_pos: field_pos.0 - 1 });
        }
    }
}

fn action_button_click(
    mut interaction_query: Query<
        (&Interaction, &ActionButton),
        (Changed<Interaction>, With<Button>),
    >,
    q: Query<(Entity, &FieldPosition, &Sprite), With<Player>>,
    mut ev_damage: EventWriter<DamageActionEvent>,
) {
    for (interaction, _action) in &mut interaction_query {
        let Interaction::Clicked = *interaction else {
            continue;
        };
        let Ok((_entity_id, field_pos, sprite)) = q.get_single() else {
            return;
        };

        let signum: isize = if sprite.flip_x { -1 } else { 1 };
        let target_field_pos = field_pos.0 as isize + (1 * signum);

        if target_field_pos >= 0 && target_field_pos < MAX_FIELDS as isize {
            ev_damage.send(DamageActionEvent { target_pos: target_field_pos as usize, damage: 1 });
        }
    }
}

fn battlefield_button_states(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
