pub mod error;
pub mod llama;

use error::Result;
use xin::chat::ChatCompletionRequestMessage;

pub trait BuildPrompt {
    fn build(messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<String>;
}
