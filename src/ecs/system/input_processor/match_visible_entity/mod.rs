use super::*;
use anyhow::Error as AnyError;

impl<'a> InputProcessor {
  /// Match any visible entity with specific text.
  pub fn match_visible_entity(&mut self, input: &str, data: &mut Data<'a>) -> Result<Entity, AnyError> {
    info!("Attempting to match a visible entity with the text '{}'", input);
    let player_id = data.player_resource.0.unwrap();
    let player = data.entities.entity(player_id.0);
    let mut result = Err(anyhow!("Not found"));
    if let Some(current_room) = get_current_room_id!(data, player) {
      info!("Examining visible objects in room {:?}", current_room);
      if let Some((target_entity, _is_in_room, _has_name, _has_brief_description, _)) = (
        &data.entities,
        &data.is_in_room,
        &data.has_name,
        &data.has_brief_description,
        !&data.is_a_player,
      )
        .join()
        .filter(|(_entity, is_in_room, _has_name, _has_brief_description, _)| is_in_room.0 == current_room)
        .filter(|(_entity, _is_in_room, has_name, _has_brief_description, _)| {
          has_name.0.to_lowercase() == input.to_lowercase()
        })
        .collect::<Vec<_>>()
        .first()
      {
        info!("Found at least one candidate visible entity: {:?}", _has_name);
        result = Ok(*target_entity);
      }
    }
    result
  }
}
