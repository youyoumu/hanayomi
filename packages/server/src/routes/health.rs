use crate::util::response::{HandlerResult, success};

pub async fn status() -> HandlerResult<String> {
    success("Ok".to_string())
}
