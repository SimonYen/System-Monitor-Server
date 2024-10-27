use salvo::prelude::*;
use serde::Serialize;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

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
        let mut s =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        //构造对应的数组
        let mut vec_name: Vec<String> = Vec::new();
        let mut vec_vendor: Vec<String> = Vec::new();
        let mut vec_frequency: Vec<u64> = Vec::new();
        let mut vec_usage: Vec<f32> = Vec::new();
        let mut vec_brand: Vec<String> = Vec::new();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        //再次刷新，因为使用率计算需要一定时间采样
        s.refresh_cpu_all();
        //遍历
        for cpu in s.cpus() {
            vec_name.push(cpu.name().to_string());
            vec_vendor.push(cpu.vendor_id().to_string());
            vec_frequency.push(cpu.frequency());
            vec_usage.push(cpu.cpu_usage());
            vec_brand.push(cpu.brand().to_string());
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
