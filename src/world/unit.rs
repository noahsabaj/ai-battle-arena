use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::prelude::Parent;
use bevy::ui::{Style, Val, PositionType, UiRect};
use bevy::text::{TextBundle, TextStyle, TextSection};
use bevy::asset::AssetServer;
use std::collections::HashSet;
use crate::world::actions::Action;
use crate::world::HexCoord;
use crate::ai::{AiController, RandomAi};

#[derive(Resource)]
struct GameAI(RandomAi);

#[derive(Resource)] struct TurnState { turn: u32, time: f32 }
#[derive(Component)] struct TurnText;
#[derive(Component)] struct CountText;
#[derive(Component)] struct BannerText;

/* -------------------------  DATA  ------------------------- */

#[derive(Component)]
pub struct Unit {
    pub team: Team,
    pub unit_type: UnitType,
    pub health: f32,
    pub max_health: f32,
}

#[derive(Component)]
pub struct HexPosition { pub coord: HexCoord }

#[derive(Component)] pub struct Dead;

#[derive(Component)]
pub struct HealthBar;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Team { Red, Blue }

#[derive(Clone, Copy, Debug)]
pub enum UnitType { Worker, Fighter, Scout }

/* colours for Bevy + short log tags */
impl Team {
    pub fn color(&self) -> Color {
        match self {
            Team::Red  => Color::rgb(0.8, 0.2, 0.2),
            Team::Blue => Color::rgb(0.2, 0.2, 0.8),
        }
    }
    pub fn tag(&self) -> &'static str {
        match self {
            Team::Red  => "[RED]",
            Team::Blue => "[BLUE]",
        }
    }
}

/* -------------------------  PLUGIN  ------------------------- */

pub struct UnitsPlugin;
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_initial_units, spawn_ui))
            .add_systems(
               Update,
               (
                    game_turn_system.run_if(not(game_over)),
                    update_unit_positions,
                    update_health_bars,
                    remove_dead_units,
                    check_victory,
                    update_ui_system,
               ).chain(),
            )
            .insert_resource(GameOver(false))  
            .insert_resource(TurnState { turn: 0, time: 0.0 })
            .insert_resource(GameAI(RandomAi::default()));
    }
}

/* ---------------------  SET-UP HELPERS  --------------------- */
fn spawn_initial_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let unit_mesh = meshes.add(Circle::new(15.0));
    println!("[START] GAME START - Spawning units!");
    println!("----------------------------------------");
    for i in 0..6 {
        let coord = HexCoord { q: -8 + i*2, r: -5 };
        println!("[RED]  Spawning RED  Fighter {} at ({}, {})", i+1, coord.q, coord.r);
        spawn_unit(
            &mut commands,
            coord,
            Team::Red,
            UnitType::Fighter,
            unit_mesh.clone(),
            materials.add(ColorMaterial::from(Team::Red.color())),
        );
    }
    for i in 0..6 {
        let coord = HexCoord { q: -8 + i*2, r:  5 };
        println!("[BLUE] Spawning BLUE Fighter {} at ({}, {})", i+1, coord.q, coord.r);
        spawn_unit(
            &mut commands,
            coord,
            Team::Blue,
            UnitType::Fighter,
            unit_mesh.clone(),
            materials.add(ColorMaterial::from(Team::Blue.color())),
        );
    }
    println!("----------------------------------------");
}

fn spawn_unit(
    commands: &mut Commands,
    coord: HexCoord,
    team: Team,
    unit_type: UnitType,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) {
    let world = hex_to_world_pos(coord.q, coord.r);
    let parent_id = commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            transform: Transform::from_translation(Vec3::new(world.x, world.y, 1.0)),
            ..default()
        },
        Unit {
            team,
            unit_type,
            health: 100.0,
            max_health: 100.0,
        },
        HexPosition { coord },
    )).id();

    let bar_id = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(60.0, 8.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 28.0, 3.0)),
            ..default()
        },
        HealthBar,
    )).id();

    commands.entity(parent_id).add_child(bar_id);
}
/* ---------------------  GAME-TURN SYSTEM  ------------------- */

#[derive(Resource)] struct GameOver(bool);

fn game_turn_system(
    time: Res<Time>,
    mut commands: Commands,
    mut units: Query<(Entity, &mut Unit, &mut HexPosition), Without<Dead>>,
    mut last_turn: Local<f32>,
    mut turn: Local<u32>,
    mut turn_state: ResMut<TurnState>,
) {
    // 2-second tick
    let now = time.elapsed_seconds();
    if now - *last_turn < 2.0 {
        return;
    }
    *last_turn = now;
    *turn += 1;
    turn_state.turn = *turn;
    turn_state.time = now;
    println!("\n----------------------------------------");
    println!("[TURN] TURN {} - Time: {:.1}s", *turn, now);

    /* movement ---------------------------------------------------------- */
    let mut occupied : HashSet<(i32,i32)> = units.iter().map(|(_,_,p)| (p.coord.q,p.coord.r)).collect();
    let mut moves = Vec::new();

    for (e, unit, mut pos) in &mut units {
        let old = pos.coord;
        let (dq,dr) = if pos.coord.r > 0 { (0,-1) }     // move toward centre
                      else if pos.coord.r < 0 { (0,1) }
                      else {
                          const DIRS:[(i32,i32);6]=[(1,0),(1,-1),(0,-1),(-1,0),(-1,1),(0,1)];
                          DIRS[(*turn as usize + e.index() as usize) % 6]
                      };
        let next = (old.q+dq, old.r+dr);
        if next.0.abs()<10 && next.1.abs()<8 && !occupied.contains(&next){
            occupied.remove(&(old.q,old.r)); occupied.insert(next);
            pos.coord.q = next.0; pos.coord.r = next.1;
            moves.push((unit.team, old, pos.coord));
        }
    }

    println!("[MOVE] Movement Phase:");
    for (t,o,n) in moves {
        println!("   {} {:?}: ({}, {})  ({}, {})", t.tag(), t, o.q,o.r, n.q,n.r);
    }

    /* combat ------------------------------------------------------------ */
    println!("\n[COMBAT] Combat Phase:");
    let snapshot: Vec<_> = units.iter().map(|(e,u,p)|(e,u.team,p.coord)).collect();
    let mut fights = Vec::new();

    for i in 0..snapshot.len() {
        for j in (i+1)..snapshot.len() {
            let (e1,t1,c1) = snapshot[i]; let (e2,t2,c2) = snapshot[j];
            if t1!=t2 && hex_distance(c1,c2)<=1 { fights.push((e1,e2,c1,c2)); }
        }
    }

    if fights.is_empty() { println!("   No combat this turn"); }
    else {
        for (e1,e2,c1,c2) in fights {
            if let Ok([(ent1,mut u1,_),(ent2,mut u2,_)]) = units.get_many_mut([e1,e2]) {
                let dmg=35.0; let h1=u1.health; let h2=u2.health;
                u1.health-=dmg; u2.health-=dmg;
                println!("   {} {:?} at ({}, {}): {:.0}  {:.0} HP",
                         u1.team.tag(),u1.team,c1.q,c1.r,h1,u1.health.max(0.0));
                println!("   {} {:?} at ({}, {}): {:.0}  {:.0} HP",
                         u2.team.tag(),u2.team,c2.q,c2.r,h2,u2.health.max(0.0));
                if u1.health<=0.0 { commands.entity(ent1).insert(Dead);
                                    println!("     {} {:?} Fighter defeated!",u1.team.tag(),u1.team); }
                if u2.health<=0.0 { commands.entity(ent2).insert(Dead);
                                    println!("     {} {:?} Fighter defeated!",u2.team.tag(),u2.team); }
            }
        }
    }

    /* status ping every 5 turns ---------------------------------------- */
    if *turn % 5 == 0 {
        let (reds,blues):(Vec<_>,Vec<_>) = units.iter()
            .map(|(_,u,p)|(u.team,p.coord,u.health))
            .partition(|(t,_,_)| *t==Team::Red);

        println!("\n[STATUS] Status Report:");
        println!("   [RED]  {} units", reds.len());
        for (i,(_,c,h)) in reds.iter().enumerate()  { println!("      {} ({}, {})  {:.0} HP", i+1,c.q,c.r,h); }
        println!("   [BLUE] {} units", blues.len());
        for (i,(_,c,h)) in blues.iter().enumerate() { println!("      {} ({}, {})  {:.0} HP", i+1,c.q,c.r,h); }
    }
}

fn update_unit_positions(
    mut q: Query<(&HexPosition, &mut Transform), (Changed<HexPosition>, Without<Dead>)>,
) {
    for (hex, mut tf) in &mut q {
        let world = hex_to_world_pos(hex.coord.q, hex.coord.r);
        tf.translation.x = world.x;
        tf.translation.y = world.y;
    }
}

fn remove_dead_units(
    mut commands: Commands,
    dead: Query<Entity, With<Dead>>,
) {
    for e in &dead {
        commands.entity(e).despawn();
    }
}

/* victory --------------------------------------------------- */
fn check_victory(
    units: Query<&Unit, Without<Dead>>,
    mut over: ResMut<GameOver>,
) {
    if over.0 {
        return;
    }
    let (mut r, mut b) = (0, 0);
    for u in &units {
        match u.team {
            Team::Red  => r += 1,
            Team::Blue => b += 1,
        }
    }
    match (r, b) {
        (0, 0) => {
            println!("\n[DRAW] DRAW! Both teams eliminated!");
            over.0 = true;
        }
        (0, _) => {
            println!("\n[BLUE] BLUE TEAM WINS!");
            over.0 = true;
        }
        (_, 0) => {
            println!("\n[RED]  RED TEAM WINS!");
            over.0 = true;
        }
        _ => {}
    }
}

fn game_over(over: Res<GameOver>) -> bool {
    over.0
}

/* --------------------------------------------------------------------------
   HEALTH-BAR SYNC
---------------------------------------------------------------------------*/
fn update_health_bars(
    mut bars: Query<(&Parent, &mut Sprite), With<HealthBar>>,
    units: Query<&Unit>,
) {
    for (parent, mut sprite) in &mut bars {
        if let Ok(unit) = units.get(parent.get()) {
            let ratio = (unit.health / unit.max_health).clamp(0.0, 1.0);
            // scale width
            if let Some(size) = sprite.custom_size.as_mut() {
                size.x = 60.0 * ratio;
            }
            // colour gradient
            sprite.color = if ratio > 0.5 {
                Color::GREEN
            } else if ratio > 0.25 {
                Color::rgb(1.0, 0.9, 0.0)
            } else {
                Color::RED
            };
        }
    }
}

// ─────────────────────  UI OVERLAY ─────────────────────

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Turn & Time
    commands.spawn((
        TextBundle {
            text: TextSection {
                value: "Turn: 0 | Time: 0.0s".into(),
                style: TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(15.),
                top:  Val::Px(15.),
                ..Default::default()
            },
            ..Default::default()
        },
        TurnText,
    ));
    // Live counts
    commands.spawn((
        TextBundle {
            text: TextSection {
                value: "Red: 0 • Blue: 0".into(),
                style: TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(15.),
                top:  Val::Px(50.),
                ..Default::default()
            },
            ..Default::default()
        },
        CountText,
    ));
}

fn update_ui_system(
    turn_state: Res<TurnState>,
    units: Query<&Unit>,
    mut turn_text: Query<&mut Text, With<TurnText>>,
    mut count_text: Query<&mut Text, With<CountText>>,
) {
    for mut txt in &mut turn_text {
        txt.sections[0].value = format!("Turn: {} | Time: {:.1}s",
            turn_state.turn, turn_state.time);
    }
    let (r,b) = units.iter()
        .fold((0,0), |(r,b), u| if u.team == Team::Red { (r+1,b) } else { (r,b+1) });
    for mut txt in &mut count_text {
        txt.sections[0].value = format!("Red: {} • Blue: {}", r, b);
    }
}

/// When the game ends, spawn a big banner in the centre
fn banner_system(
    over: Res<GameOver>,
    units: Query<&Unit>,
    mut done: Local<bool>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if over.0 && !*done {
        *done = true;
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let (r,b) = units.iter()
            .fold((0,0), |(r,b), u| if u.team==Team::Red { (r+1,b) } else { (r,b+1) });
        let msg = match (r,b) {
            (0,0) => "DRAW!".into(),
            (0,_) => "BLUE TEAM WINS!".into(),
            (_,0) => "RED  TEAM WINS!".into(),
            _     => unreachable!(),
        };
        commands.spawn((
            TextBundle {
                text: TextSection {
                    value: msg,
                    style: TextStyle {
                        font,
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    left:  Val::Percent(35.0),
                    top:   Val::Percent(45.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            BannerText,
        ));
    }
}

fn hex_distance(a: HexCoord, b: HexCoord) -> i32 {
    ((a.q - b.q).abs() + (a.q + a.r - b.q - b.r).abs() + (a.r - b.r).abs()) / 2
}

fn hex_to_world_pos(q: i32, r: i32) -> Vec2 {
    const SZ: f32 = 30.0;
    Vec2::new(
        SZ * (f32::sqrt(3.0) * q as f32 + f32::sqrt(3.0) / 2.0 * r as f32),
        SZ * (3.0 / 2.0 * r as f32),
    )
}
