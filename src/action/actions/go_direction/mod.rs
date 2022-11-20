use crate::action::Actionable;
use crate::ecs::entity::EntityId;
use crate::ecs::entity::RoomId;
use crate::ecs::system::action_processor::Data;
use crate::effect::*;
use crate::map::Direction;
use crate::map::PassageDestination;
use anyhow::Error as AnyError;

/// The `GoDirection` action.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GoDirection {
  pub entity_id: EntityId,
  pub direction: Direction,
}

impl GoDirection {
  pub fn get_room_id(&self, data: &mut Data) -> Result<RoomId, AnyError> {
    let entity = get_entity!(data, self.entity_id);
    let room_id_option = get_current_room_id!(data, entity);
    if room_id_option.is_none() {
      return Err(anyhow!("you are unable to move in that direction"));
    }
    let room_id = room_id_option.unwrap();
    Ok(room_id)
  }

  pub fn get_destination_id(&self, data: &mut Data) -> Result<RoomId, AnyError> {
    let room_id = self.get_room_id(data)?;
    let room = get_entity!(data, room_id);
    let passage_option = get_passage_to!(data, room, &self.direction);
    if passage_option.is_none() {
      return Err(anyhow!("you are unable to move in that direction"));
    }
    let passage = passage_option.unwrap();
    if passage.to.is_message() {
      return Err(anyhow!("{:#?}", passage.to));
    }
    let destination_id = if let PassageDestination::Room(destination_id) = passage.to {
      destination_id
    } else {
      unreachable!()
    };
    Ok(destination_id)
  }
}

impl Actionable for GoDirection {
  fn get_effects(&self, data: &mut Data) -> Result<Vec<Effect>, AnyError> {
    let room_id = self.get_room_id(data)?;
    let destination_id = self.get_destination_id(data)?;
    Ok(vec![
      Effect::EntityWalksOutOfRoom(EntityWalksOutOfRoom {
        entity_id: self.entity_id,
        direction: self.direction,
        room_id,
      }),
      Effect::EntityWalksIntoRoom(EntityWalksIntoRoom {
        entity_id: self.entity_id,
        direction: self.direction.get_inverse(),
        room_id: destination_id,
      }),
      Effect::EntityLooksAround(EntityLooksAround {
        entity_id: self.entity_id,
      }),
      Effect::EntitySetInitiative(EntitySetInitiative {
        entity_id: self.entity_id,
        value: 0,
      }),
    ])
  }

  fn can_execute(&self, data: &mut Data) -> Result<(), AnyError> {
    self.get_destination_id(data)?;
    Ok(())
  }
}
