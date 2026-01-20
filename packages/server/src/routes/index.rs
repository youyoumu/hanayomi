use crate::util::response::{HandlerResult, success};

pub async fn index() -> HandlerResult<String> {
    success("Hello, world!".to_string())
}
