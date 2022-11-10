#[macro_export]
macro_rules! create_room {
  ($system_data: expr, $name: expr, $description: expr) => {{
    use $crate::ecs::components::*;
    let room = $system_data.entities.create();
    $system_data
      .has_name
      .insert(room, HasName($name.into()))
      .expect(&format!("Unable to insert name {} for entity!", $name));
    $system_data
      .has_description
      .insert(
        room,
        HasDescription {
          initial: None,
          brief: $description.into(),
        },
      )
      .expect(&format!("Unable to insert description {} for entity!", $description));
    $system_data
      .has_passages
      .insert(room, HasPassages::default())
      .expect("Unable to insert exits for entity!");
    $system_data
      .is_a_room
      .insert(room, IsARoom)
      .expect("Unable to insert is-a-room for entity!");
    room
  }};
}

#[macro_export]
macro_rules! format_room {
  ($system_data: expr, $room: expr) => {{
    use $crate::ecs::entity::RoomId;
    let mut string = String::new();
    if let Some(name) = get_name!($system_data, $room) {
      string.push_str(format!("<bold>{}<reset>\n", name).as_str());
    }
    if let Some(description) = get_brief_description!($system_data, $room) {
      string.push_str(format!("{}\n", description).as_str());
    }
    {
      let room_id = RoomId($room.id());
      for (_entities, _is_in_room, _is_an_object, has_description) in (
        &$system_data.entities,
        &$system_data.is_in_room,
        &$system_data.is_an_object,
        &$system_data.has_description,
      )
        .join()
        .filter(|(_entity, is_in_room, _is_an_object, _has_description)| is_in_room.0 == room_id)
      {
        string.push_str(format!("<fg_ext180>{}<reset>\n", has_description.brief).as_str());
      }
    }
    {
      let room_id = RoomId($room.id());
      for (_entities, _is_in_room, _is_an_actor, has_description, _is_a_player) in (
        &$system_data.entities,
        &$system_data.is_in_room,
        &$system_data.is_an_actor,
        &$system_data.has_description,
        !&$system_data.is_a_player,
      )
        .join()
        .filter(|(_entity, is_in_room, _is_an_actor, _has_description, _)| is_in_room.0 == room_id)
      {
        string.push_str(format!("<fg_ext162>{}<reset>\n", has_description.brief).as_str());
      }
    }
    if let Some(passages) = get_passages!($system_data, $room) {
      string.push_str(format!("<green>{}<reset>\n", passages).as_str());
    }
    format!("{}", string)
  }};
}