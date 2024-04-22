use std::slice::Iter;

use crate::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_WIDTH, Z_LEVELS};
use crate::utils::map_to_world;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, sprite::Anchor};
use ndarray::prelude::*;

use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm, MultiFractal, Perlin,
};

pub struct TerrainPlugin;

#[derive(Resource)]
pub struct StoneMap(Array<Option<Entity>, Ix3>);

fn insert_stone_map(mut commands: Commands) {
    commands.insert_resource(StoneMap(Array::from_elem(
        (Z_LEVELS as usize, MAP_WIDTH as usize, MAP_HEIGHT as usize),
        None,
    )));
}

#[derive(Component)]
pub struct Layer(i8);

#[derive(Component)]
pub struct Position(IVec3);

#[derive(Component)]
pub struct Stone;

#[derive(Component)]
pub struct Item;

#[derive(Bundle)]
pub struct TileBundle {
    position: Position,
    layer: Layer,
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle)]
pub struct StoneBundle {
    item: Item,
    stone: Stone,
    position: Position,
    sprite: SpriteBundle,
    layer: Layer,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn into_iter() -> core::array::IntoIter<Self, 4> {
        [Self::North, Self::South, Self::East, Self::West].into_iter()
    }

    pub fn to_ivec2(self) -> IVec2 {
        match self {
            Self::North => IVec2::new(0, 1),
            Self::South => IVec2::new(0, -1),
            Self::East => IVec2::new(1, 0),
            Self::West => IVec2::new(-1, 0),
        }
    }
}

fn get_stone_zlevel(
    query: Query<&Position, (With<Stone>, Changed<Position>)>,
    stone_map: Res<StoneMap>,
) {
    if query.iter().count() == 0 {
        return;
    }

    let mut level_edges = Vec::<[IVec2; 2]>::new();

    for position in query.iter() {
        let [x, y, z] = position.0.to_array();
        if z != Z_LEVELS - 3 {
            continue;
        }

        for direction_name in Direction::into_iter() {
            let [dx, dy] = Direction::to_ivec2(direction_name).to_array();
            let (nx, ny) = (x + dx, y + dy);

            if !(0..{ MAP_WIDTH }).contains(&nx)
                || !(0..{ MAP_HEIGHT }).contains(&ny)
                || stone_map.0[(z as usize, nx as usize, ny as usize)].is_none()
            {
                match direction_name {
                    Direction::East => {
                        level_edges.push([IVec2::new(x + 1, y), IVec2::new(x + 1, y + 1)])
                    }
                    Direction::West => level_edges.push([IVec2::new(x, y), IVec2::new(x, y + 1)]),
                    Direction::South => level_edges.push([IVec2::new(x, y), IVec2::new(x + 1, y)]),
                    Direction::North => {
                        level_edges.push([IVec2::new(x, y + 1), IVec2::new(x + 1, y + 1)])
                    }
                }
            }
        }
    }

    println!("{:?}", level_edges);
}

fn set_stone_visibility(
    mut query: Query<(&Position, &mut Visibility), (With<Stone>, Changed<Position>)>,
    stone_map: Res<StoneMap>,
) {
    for (position, mut visibility) in query.iter_mut() {
        let [x, y, z] = position.0.to_array();

        let stone_above = stone_map.0.get((z as usize + 1, x as usize, y as usize));

        match stone_above {
            Some(Some(_)) => {
                *visibility = Visibility::Hidden;
            }
            _ => {
                *visibility = Visibility::Inherited;
            }
        }
    }
}

fn create_stone(image_handle: Handle<Image>, position: IVec3) -> StoneBundle {
    StoneBundle {
        item: Item,
        stone: Stone,
        position: Position(position),
        layer: Layer(0),
        sprite: SpriteBundle {
            texture: image_handle,
            transform: Transform::from_translation(
                map_to_world(position)
                    .extend(-(position.y + position.x) as f32 + position.z as f32 * 0.1),
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_WIDTH, TILE_WIDTH)),
                anchor: Anchor::Custom(Vec2::new(0.5, 0.7)),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

fn generate_noise() -> NoiseMap {
    let fbm = Fbm::<Perlin>::default()
        .set_octaves(3)
        .set_persistence(0.1)
        .set_frequency(0.05);
    PlaneMapBuilder::new(&fbm)
        .set_size(MAP_WIDTH as usize, MAP_HEIGHT as usize)
        .set_x_bounds(0.0, MAP_WIDTH as f64)
        .set_y_bounds(0.0, MAP_HEIGHT as f64)
        .build()
}

pub fn spawn_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut stone_map: ResMut<StoneMap>,
) {
    let stone_texture_handle = asset_server.load("sprites/terrain/stone.png");
    let width = MAP_WIDTH;
    let height = MAP_HEIGHT;

    let noise = generate_noise();

    for x in 0..(width) {
        for y in 0..(height) {
            let z: i32 = ((0.2 * Z_LEVELS as f64
                + (noise.get_value(x as usize, y as usize) + 1.0) * 0.4 * Z_LEVELS as f64)
                .min(Z_LEVELS as f64 - 1.0)) as i32;

            for z_level in 0..(z) {
                let stone_entity = commands
                    .spawn(create_stone(
                        stone_texture_handle.clone(),
                        IVec3::new(x, y, z_level),
                    ))
                    .id();

                stone_map.0[(z_level as usize, x as usize, y as usize)] = Some(stone_entity);
            }
        }
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (insert_stone_map, spawn_terrain).chain())
            .add_systems(Update, (set_stone_visibility, get_stone_zlevel));
    }
}
