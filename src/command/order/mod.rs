use super::Command;
use crate::action::Action;
use crate::command::Commandable;
use crate::ecs::entity::PlayerId;
use crate::ecs::system::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `Order` command.
#[derive(Clone, Debug)]
pub struct Order {
  pub player_id: PlayerId,
  pub command: Box<Command>,
}

impl Commandable for Order {
  fn get_action(&self, data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    self.command.get_action(data)
  }
}
