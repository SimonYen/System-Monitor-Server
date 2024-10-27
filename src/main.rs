mod router;
mod handler;

use salvo::prelude::*;
use salvo::logging::Logger;
use sysinfo::System;
use lazy_static::lazy_static;
use std::sync::{Arc,Mutex};

lazy_static!{
    static ref SYSTEM:Arc<Mutex<System>> = Arc::new(Mutex::new(System::new()));
}




#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = router::get_router();
    let service=Service::new(router).hoop(Logger::new());
    let acceptor = TcpListener::new("localhost:9999").bind().await;

    Server::new(acceptor).serve(service).await;
}