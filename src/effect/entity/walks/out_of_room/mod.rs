use crate::ecs::entity::EntityId;
use crate::ecs::entity::RoomId;
use crate::ecs::system::effect_processor::Data as EffectProcessorData;
use crate::effect::Effectable;
use crate::map::Direction;
use anyhow::Error;

/// `EntityWalksOutOfRoom`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OutOfRoom {
  /// The entity performing the action.
  pub entity_id: EntityId,
  /// The room the entity exits.
  pub room_id: RoomId,
  /// The direction to which the entity leaves.
  pub direction: Direction,
}

impl Effectable for OutOfRoom {
  fn process(&self, data: &mut EffectProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    let name = get_name!(data, entity).unwrap();
    they!(
      data,
      entity,
      format!("{} walks out to the {}.", name, self.direction.get_lowercase())
    );
    remove_is_in_room!(data, entity);
    Ok(())
  }
}
