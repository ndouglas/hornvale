use crate::entity::EntityId;
use crate::entity::RoomId;
use crate::map::Direction;
use crate::system::effect_processor::Data as EffectProcessorData;
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

impl OutOfRoom {
  pub fn process(&self, data: &mut EffectProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    let room = get_entity!(data, self.room_id);
    let name = get_name!(data, entity).unwrap();
    remove_is_in_room!(data, entity);
    write_output_3rd!(
      data,
      entity,
      room,
      format!("{} walks out to the {}.", name, self.direction.get_lowercase())
    );
    Ok(())
  }
}
