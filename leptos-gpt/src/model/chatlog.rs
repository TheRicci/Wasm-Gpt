use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatLog {
    pub chat: Vec<Log>
}
impl ChatLog {
    pub fn new() -> ChatLog{
        ChatLog {
            chat: vec![Log{user:false,message:"How can i help you today?".to_owned()}],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Log {
    pub user: bool,
    pub message: String,
}
