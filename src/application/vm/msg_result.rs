#[derive(Debug,  Clone)]
pub struct MsgResult {
    pub message: String,
    pub success: bool,
}

impl MsgResult {
    pub fn new(message: String, success: bool) -> MsgResult {
        MsgResult {
            message,
            success,
        }
    }
}