use specs::prelude::*;

pub mod has_ai;
pub use has_ai::HasAi;
pub mod has_brief_description;
pub use has_brief_description::HasBriefDescription;
pub mod has_gender;
pub use has_gender::HasGender;
pub mod has_initiative;
pub use has_initiative::HasInitiative;
pub mod has_intent;
pub use has_intent::HasIntent;
pub mod has_name;
pub use has_name::HasName;
pub mod has_passages;
pub use has_passages::HasPassages;
pub mod has_state;
pub use has_state::HasState;
pub mod is_a_player;
pub use is_a_player::IsAPlayer;
pub mod is_a_room;
pub use is_a_room::IsARoom;
pub mod is_a_spawn_room;
pub use is_a_spawn_room::IsASpawnRoom;
pub mod is_an_actor;
pub use is_an_actor::IsAnActor;
pub mod is_an_object;
pub use is_an_object::IsAnObject;
pub mod is_in_room;
pub use is_in_room::IsInRoom;

pub fn register_components(ecs: &mut World) {
  ecs.register::<HasAi>();
  ecs.register::<HasBriefDescription>();
  ecs.register::<HasGender>();
  ecs.register::<HasInitiative>();
  ecs.register::<HasIntent>();
  ecs.register::<HasName>();
  ecs.register::<HasPassages>();
  ecs.register::<HasState>();
  ecs.register::<IsAnActor>();
  ecs.register::<IsAPlayer>();
  ecs.register::<IsARoom>();
  ecs.register::<IsASpawnRoom>();
  ecs.register::<IsAnObject>();
  ecs.register::<IsInRoom>();
}
