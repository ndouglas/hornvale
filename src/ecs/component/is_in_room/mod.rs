use specs::prelude::*;

use crate::ecs::entity::RoomId;

/// The `IsInRoom` type.
#[derive(Clone, Component, Debug, Default, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[repr(transparent)]
pub struct IsInRoom(pub RoomId);
