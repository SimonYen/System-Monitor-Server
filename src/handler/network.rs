use crate::NETWORKS;
use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct Data {
    received: u64,                    //当前接收到的字节数
    total_received: u64,              //总共接收到的字节数
    transmitted: u64,                 //当前传输的字节数
    total_transmitted: u64,           //总共传输的字节数
    packets_received: u64,            //接收到的包数
    total_packets_received: u64,      //总共接收到的包数
    packets_transmitted: u64,         //传输的包数
    total_packets_transmitted: u64,   //总共传输的包数
    errors_on_received: u64,          //接收到的错误包数
    total_errors_on_received: u64,    //总共接收到的错误包数
    errors_on_transmitted: u64,       //传输的错误包数
    total_errors_on_transmitted: u64, //总共传输的错误包数
    mac_address: String,              //mac地址
    ip_address: Vec<String>,          //ip地址，可能有多个
}

#[derive(Serialize)]
struct NetworkInterface {
    name: String, //网络接口名
    data: Data,
}

#[derive(Serialize)]
struct Networks {
    interfaces: Vec<NetworkInterface>,
}

impl Networks {
    fn new() -> Self {
        let mut net = NETWORKS.lock().unwrap();
        //刷新
        net.refresh_list();
        let mut vec_interface: Vec<NetworkInterface> = Vec::new();
        //遍历
        for (name, data) in net.list() {
            let d = Data {
                received: data.received(),
                total_received: data.total_received(),
                transmitted: data.transmitted(),
                total_transmitted: data.total_transmitted(),
                packets_received: data.packets_received(),
                total_packets_received: data.total_packets_received(),
                packets_transmitted: data.packets_transmitted(),
                total_packets_transmitted: data.total_packets_transmitted(),
                errors_on_received: data.errors_on_received(),
                total_errors_on_received: data.errors_on_received(),
                errors_on_transmitted: data.errors_on_transmitted(),
                total_errors_on_transmitted: data.total_errors_on_received(),
                mac_address: data.mac_address().to_string(),
                ip_address: data
                    .ip_networks()
                    .iter()
                    .map(|x| x.addr.to_string())
                    .collect(),
            };
            vec_interface.push(NetworkInterface {
                name: name.to_string(),
                data: d,
            });
        }
        Networks {
            interfaces: vec_interface,
        }
    }
}

#[handler]
pub async fn get_all_info(res: &mut Response) {
    let nets = Networks::new();
    res.render(Json(nets));
}
