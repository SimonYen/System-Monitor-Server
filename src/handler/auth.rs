use salvo::prelude::*;

#[handler]
pub async fn hello() -> &'static str {
    "Hello World 颜"
}
