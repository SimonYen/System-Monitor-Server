mod handler;
mod router;

use lazy_static::lazy_static;
use salvo::logging::Logger;
use salvo::prelude::*;
use std::sync::{Arc, Mutex};
use sysinfo::{Disks, Networks, System};
use tera::Tera;

lazy_static! {
    static ref SYSTEM: Arc<Mutex<System>> = Arc::new(Mutex::new(System::new()));
    static ref DISKS: Arc<Mutex<Disks>> = Arc::new(Mutex::new(Disks::new()));
    static ref NETWORKS: Arc<Mutex<Networks>> = Arc::new(Mutex::new(Networks::new()));
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::new("templates/**/*.html").unwrap();
        tera.full_reload().unwrap();
        tera
    };
}

//解析命令行参数
use argh::FromArgs;

#[derive(FromArgs)]
/// A System monitor written in Rust!
struct Args {
    /// enable logging
    #[argh(switch, short = 'l')]
    log: bool,
    /// address
    #[argh(option, short = 'a')]
    address: Option<String>,
    /// port
    #[argh(option, short = 'p')]
    port: Option<u32>,
}

#[tokio::main]
async fn main() {
    //获取命令行参数
    let args: Args = argh::from_env();
    let router = router::get_router();
    let ip = format!(
        "{}:{}",
        args.address.unwrap_or("127.0.0.1".to_string()),
        args.port.unwrap_or(9999)
    );
    if args.log {
        tracing_subscriber::fmt().init();
        let service = Service::new(router).hoop(Logger::new());
        let acceptor = TcpListener::new(&ip).bind().await;
        Server::new(acceptor).serve(service).await;
    } else {
        let acceptor = TcpListener::new(&ip).bind().await;
        Server::new(acceptor).serve(router).await;
    }
}
