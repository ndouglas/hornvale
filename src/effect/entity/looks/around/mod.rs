use crate::ecs::entity::EntityId;
use crate::ecs::system::effect_processor::Data;
use crate::effect::Effectable;
use anyhow::Error;

/// `EntityLooksAround`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LooksAround {
  /// The entity performing the action.
  pub entity_id: EntityId,
}

impl Effectable for LooksAround {
  fn process(&self, data: &mut Data) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    let room_id = get_current_room_id!(data, entity).unwrap();
    let room = get_entity!(data, room_id);
    let name = get_name!(data, entity).unwrap();
    they!(data, entity, format!("{} looks around.", name));
    show!(data, entity, format_room!(data, room));
    reset_state!(data, entity, 1);
    reset_state!(data, entity, 0);
    Ok(())
  }
}
