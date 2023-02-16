use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
#[derive(Resource)]
pub struct PileSpawnList {
    to_spawn: Vec<Pile>,
}
pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PileSpawnList {
            to_spawn: vec![Pile {
                pile_type: PileType::Carbohydrate,
                pile_count: PileCount::HasResources(500),
            }],
        })
        .add_startup_system(pile_spawner)
        .add_system(pile_despawner);
    }
}

enum PileType {
    Carbohydrate,
    Protein,
}
#[derive(PartialEq, Eq)]
enum PileCount {
    HasResources(u32),
    Empty,
}

// contains the piles resource type and count
#[derive(Component)]
pub struct Pile {
    pile_type: PileType,
    pile_count: PileCount,
}
impl Pile {
    // harvest pile, return true if it can be harvested
    fn harvest(&mut self, amount: u32) -> bool {
        // get the resource count
        if let PileCount::HasResources(mut count) = self.pile_count {
            match count.checked_sub(amount) {
                Some(new_count) => {
                    // set new count
                    count = new_count;
                    return true;
                }
                None => {
                    self.pile_count = PileCount::Empty;
                }
            }
        }
        return false;
    }
}

// spawns piles that are added to the spawn list
fn pile_spawner(mut commands: Commands, mut piles: ResMut<PileSpawnList>) {
    while !piles.to_spawn.is_empty() {
        let pile = piles.to_spawn.pop().unwrap();

        // get the resource count
        if let PileCount::HasResources(count) = pile.pile_count {
            let radius = (count as f32).powf(1.0 / 3.0);
            let color = match pile.pile_type {
                PileType::Carbohydrate => Color::YELLOW,
                PileType::Protein => Color::BLUE,
            };

            // spawn the pile
            commands
                .spawn(pile)
                .insert(GeometryBuilder::build_as(
                    &RegularPolygon {
                        sides: 32,
                        feature: shapes::RegularPolygonFeature::Radius(radius),
                        ..shapes::RegularPolygon::default()
                    },
                    DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(color)),
                    Transform::default(),
                ))
                .insert(Collider::ball(radius))
                .insert(Sensor);
        }
    }
}
// despawns entities whenever they run out
fn pile_despawner(mut commands: Commands, piles: Query<(Entity, &Pile), Changed<Pile>>) {
    for (e, pile) in piles.iter() {
        if pile.pile_count == PileCount::Empty {
            commands.entity(e).despawn();
        }
    }
}
