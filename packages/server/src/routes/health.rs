use crate::util::response::{HandlerResult, success};

pub async fn index() -> HandlerResult<String> {
    success("Ok".to_string())
}
