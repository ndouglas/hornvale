use crate::action::Action;
use crate::entity::PlayerId;
use crate::systems::command_processor::Data as CommandProcessorData;
use anyhow::Error;

/// The `Echo` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Echo {
  pub player_id: PlayerId,
  pub string: String,
  pub original_input: String,
}

impl Echo {
  pub fn get_action(&self, data: &mut CommandProcessorData) -> Result<Option<Action>, Error> {
    write_output!(data, self.string.clone());
    Ok(None)
  }
}
