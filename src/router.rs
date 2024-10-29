use crate::handler;
use salvo::prelude::*;

pub fn get_router() -> Router {
    Router::new()
        .push(Router::new().get(handler::auth::hello))
        .push(Router::with_path("cpu").get(handler::cpu::get_all_info))
        .push(Router::with_path("mem").get(handler::mem::get_all_info))
        .push(Router::with_path("disk").get(handler::disk::get_all_info))
        .push(Router::with_path("net").get(handler::network::get_all_info))
}
