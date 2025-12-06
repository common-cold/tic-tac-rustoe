use common::types::{LogArgs, WebSocketResponse};


pub fn prepare_log(message: String, is_error: bool) -> String {
    let response = WebSocketResponse {
        data: common::types::ResponseData::Log(LogArgs {
            message: message,
            is_error: is_error
        })
    };
    serde_json::to_string(&response).unwrap()
}