use crate::util::response::{HandlerResult, success};

pub async fn root() -> HandlerResult<String> {
    success("Hello, world!".to_string())
}
