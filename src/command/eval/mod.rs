use crate::action::Action;
use crate::ecs::entity::PlayerId;
use crate::ecs::systems::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `Eval` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Eval {
  pub player_id: PlayerId,
  pub string: String,
  pub original_input: String,
}

impl Eval {
  pub fn get_action(&self, data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    write_output!(data, self.string.clone());
    Ok(None)
  }
}
