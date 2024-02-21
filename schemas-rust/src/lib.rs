pub mod channel_instruction_generated;
pub mod execution_request_v1_generated;
pub mod status_v1_generated;
use error::ChannelSchemaError;
pub mod error;
pub use channel_instruction_generated::*;
pub use execution_request_v1_generated::*;
pub use status_v1_generated::*;
pub fn parse_ix_data<'a>(ix_data: &'a [u8]) -> Result<ChannelInstruction, ChannelSchemaError> {
    let instruction = root_as_channel_instruction(ix_data)
        .map_err(|_| ChannelSchemaError::InvalidInstruction)?;
    Ok(instruction)
}