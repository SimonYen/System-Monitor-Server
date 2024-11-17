use crate::handler;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

pub fn get_router() -> Router {
    Router::new()
        //先添加静态文件
        .push(Router::with_path("static/<**path>").get(StaticDir::new(["static/"])))
        //REST api部分
        .push(
            Router::with_path("api")
                .push(Router::with_path("cpu").get(handler::cpu::get_all_info))
                .push(Router::with_path("mem").get(handler::mem::get_all_info))
                .push(Router::with_path("disk").get(handler::disk::get_all_info))
                .push(Router::with_path("net").get(handler::network::get_all_info))
                .push(Router::with_path("gross").get(handler::gross::get_gross_info)),
        )
        //web前端部分
        .push(Router::new().push(Router::new().get(handler::view::home)))
}
