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

trait Skillable {
    fn get_action(&self, battlefield: &Battlefield, actor: &Actor) -> Option<Box<dyn Actionable + Send + Sync + 'static>>;
}

struct DamageSkill {}

impl Skillable for DamageSkill {
    fn get_action(&self, _battlefield: &Battlefield, actor: &Actor) -> Option<Box<dyn Actionable + Send + Sync + 'static>> {
        let target_field_pos = actor.position as isize + (1 * actor.facing);
        if target_field_pos < 0 && target_field_pos >= MAX_FIELDS as isize {
            return None;
        }
        
        Some(Box::new(DamageAction { damage: 1, target: target_field_pos as usize }))
    }
}

trait Actionable {
    fn get_name(&self) -> String;
    fn apply(&self, battlefield: &Battlefield, query: &mut Query<&mut Actor>);
}

struct DamageAction {
    target: usize,
    damage: isize,
}

impl Actionable for DamageAction {
    fn get_name(&self) -> String {
        "Damage".into()
    }

    fn apply(&self, battlefield: &Battlefield, query: &mut Query<&mut Actor>) {
        let Some(target_id) = battlefield.fields[self.target] else {
            return;
        };
        let Ok(mut actor) = query.get_mut(target_id) else {
            return;
        };
        actor.health -= self.damage;
    }
}

#[derive(Component)]
struct ApplyActionEvent {
    action: Box<dyn Actionable + Send + Sync + 'static>,
}

#[derive(Resource)]
struct WorldMousePos {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct ActionButton {
    skill: Box<dyn Skillable + Send + Sync + 'static>
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Actor {
    position: usize,
    health: isize,
    facing: isize,
}

#[derive(Resource)]
struct Battlefield {
    fields: [Option<Entity>; 5]
}

#[derive(Component)]
struct BattleFieldPosition(usize);

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

struct SpawnEnemyEvent {
    pos: usize,
    name: String,
    flip_x: bool,
}

fn flip_from_facing(facing: isize) -> bool {
    facing != 1
}

fn facing_from_flip(flip: bool) -> isize {
    if flip { -1 } else { 1 }
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
        .add_event::<ApplyActionEvent>()
        .add_startup_system(setup)
        .add_system(spawn_enemy_event)
        .add_system(mouse_screen_to_world)
        .add_system(battlefield_button_click)
        .add_system(action_button_click)
        .add_system(actor_move_event)
        .add_system(actor_change_facing_event)
        .add_system(apply_skill_event)
        .add_system(battlefield_button_states)
        .add_system(update_actor_debug_text)
        .add_system(cleanup_actors)
        .add_system(update_animation)
        .add_system(update_sprite_facing)
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
            Actor {
                health: 5,
                position: player_pos,
                facing: 1,
            },
            Name("Hero".into()),
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
                BattleFieldPosition(i)
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
            ActionButton { skill: Box::new(DamageSkill {}) }
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
                Name(e.name.clone()),
                Actor {
                    health: 5,
                    position: e.pos,
                    facing: facing_from_flip(e.flip_x)
                },
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
    q: Query<(&Name, &Actor), Changed<Actor>>,
    mut q_children: Query<(&mut Text, &Parent)>
) {
    for (mut text, parent) in q_children.iter_mut() {
        let Ok((name, actor)) = q.get(parent.get()) else {
            continue;
        };
        text.sections[0].value = format!("{}\n{}hp\n@{}", name.0, actor.health, actor.position);
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

fn cleanup_actors(
    mut commands: Commands,
    mut battlefield: ResMut<Battlefield>,
    q: Query<(Entity, &Actor)>,
) {
    for (entity, actor) in q.iter() {
        if actor.health <= 0 {
            commands.entity(entity).despawn_recursive();
            battlefield.fields[actor.position] = None;
        }
    }
}


fn actor_move_event(
    mut q: Query<(&mut Actor, &mut Transform)>,
    mut battlefield: ResMut<Battlefield>,
    mut ev: EventReader<ActorMovedEvent>,
) {
    for e in ev.iter() {
        if battlefield.fields[e.new_pos].is_some() {
            continue;
        }
        let Ok((mut actor, mut transform)) = q.get_mut(e.actor) else {
            continue;
        };
        battlefield.fields[e.old_pos] = None;
        battlefield.fields[e.new_pos] = Some(e.actor);
        actor.position = e.new_pos;
        transform.translation.x = BATTLEFIELD_OFFSET + e.new_pos as f32 * (SPRITE_SIZE + 2.0);
    }
}

fn actor_change_facing_event(
    mut q: Query<&mut Actor>,
    mut ev: EventReader<ActorChangeFacingEvent>,
) {
    for e in ev.iter() {
        let Ok(mut actor) = q.get_mut(e.actor) else {
            continue;
        };
        actor.facing = actor.facing * -1;
    }
}

fn update_sprite_facing(
    mut q: Query<(&mut Sprite, &Actor), Changed<Actor>>,
) {
    for (mut sprite, actor) in q.iter_mut() {
        sprite.flip_x = flip_from_facing(actor.facing);
    };
}

fn apply_skill_event(
    mut ev: EventReader<ApplyActionEvent>,
    battlefield: Res<Battlefield>, 
    mut query: Query<&mut Actor>
) {
    for e in ev.iter() {
        e.action.apply(&battlefield, &mut query);
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
        (&Interaction, &BattleFieldPosition),
        (Changed<Interaction>, With<Button>),
    >,
    q: Query<(Entity, &Actor), With<Player>>,
    mut ev_moved: EventWriter<ActorMovedEvent>,
    mut ev_facing: EventWriter<ActorChangeFacingEvent>,
) {
    for (interaction, clicked_field_pos) in &mut interaction_query {
        let Interaction::Clicked = *interaction else {
            continue;
        };
        let Ok((entity_id, actor)) = q.get_single() else {
            return;
        };
        if clicked_field_pos.0 == actor.position {
            ev_facing.send(ActorChangeFacingEvent { actor: entity_id });
        }
        if clicked_field_pos.0 > actor.position {
            ev_moved.send(ActorMovedEvent { actor: entity_id, old_pos: actor.position, new_pos: actor.position + 1 });
        }
        if clicked_field_pos.0 < actor.position {
            ev_moved.send(ActorMovedEvent { actor: entity_id, old_pos: actor.position, new_pos: actor.position - 1 });
        }
    }
}

fn action_button_click(
    mut interaction_query: Query<
        (&Interaction, &ActionButton),
        (Changed<Interaction>, With<Button>),
    >,
    battlefield: Res<Battlefield>,
    q: Query<&Actor, With<Player>>,
    mut ev_damage: EventWriter<ApplyActionEvent>,
) {
    for (interaction, action_button) in &mut interaction_query {
        let Interaction::Clicked = *interaction else {
            continue;
        };
        let Ok(actor) = q.get_single() else {
            continue;
        };
        let Some(action) = action_button.skill.get_action(&battlefield, actor) else {
            continue;
        };

        ev_damage.send(ApplyActionEvent { action });
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
