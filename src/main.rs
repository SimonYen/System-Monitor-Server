mod router;
mod handler;

use salvo::prelude::*;
use salvo::logging::Logger;
use sysinfo::{System,Disks,Networks};
use lazy_static::lazy_static;
use std::sync::{Arc,Mutex};

lazy_static!{
    static ref SYSTEM:Arc<Mutex<System>> = Arc::new(Mutex::new(System::new()));
    static ref DISKS:Arc<Mutex<Disks>> = Arc::new(Mutex::new(Disks::new()));
    static ref NETWORKS:Arc<Mutex<Networks>> = Arc::new(Mutex::new(Networks::new()));
}




#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = router::get_router();
    let service=Service::new(router).hoop(Logger::new());
    let acceptor = TcpListener::new("localhost:9999").bind().await;

    Server::new(acceptor).serve(service).await;
}