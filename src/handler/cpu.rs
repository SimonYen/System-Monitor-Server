use salvo::prelude::*;
use serde::Serialize;
use crate::SYSTEM;

#[derive(Serialize, Debug)]
struct CPU {
    name: Vec<String>,   //CPU名称
    vendor: Vec<String>, //CPU销售商代码
    brand: Vec<String>,  //CPU品牌
    frequency: Vec<u64>, //CPU频率
    usage: Vec<f32>,     //CPU使用率
}

impl CPU {
    fn new() -> Self {
        //刷新CPU相关信息
        {
            let mut sys =SYSTEM.lock().unwrap();
            sys.refresh_cpu_all();
        }
        //构造对应的数组
        let mut vec_name: Vec<String> = Vec::new();
        let mut vec_vendor: Vec<String> = Vec::new();
        let mut vec_frequency: Vec<u64> = Vec::new();
        let mut vec_usage: Vec<f32> = Vec::new();
        let mut vec_brand: Vec<String> = Vec::new();
        //遍历
        {
            let sys=SYSTEM.lock().unwrap();
            for cpu in sys.cpus() {
                vec_name.push(cpu.name().to_string());
                vec_vendor.push(cpu.vendor_id().to_string());
                vec_frequency.push(cpu.frequency());
                vec_usage.push(cpu.cpu_usage());
                vec_brand.push(cpu.brand().to_string());
            }
        }
        CPU {
            name: vec_name,
            vendor: vec_vendor,
            frequency: vec_frequency,
            usage: vec_usage,
            brand: vec_brand,
        }
    }
}

#[handler]
pub async fn get_all_info(res: &mut Response) {
    let cpu = CPU::new();
    res.render(Json(cpu));
}
