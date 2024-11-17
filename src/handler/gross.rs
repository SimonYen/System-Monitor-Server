use salvo::prelude::*;
use serde::Serialize;
use sysinfo::System;
use tera::Context;

#[derive(Serialize)]
pub struct Gross {
    //系统名
    system_name: String,
    //主机名
    host_name: String,
    //内核版本
    kernel_version: String,
    //系统版本
    os_version: String,
    //分发id
    distribution_id: String,
    //CPU架构
    cpu_arch: String,
}

impl Gross {
    pub fn new() -> Self {
        Self {
            system_name: System::name().unwrap_or("UNKNOWN".parse().unwrap()),
            host_name: System::host_name().unwrap_or("UNKNOWN".parse().unwrap()),
            os_version: System::long_os_version().unwrap_or("UNKNOWN".parse().unwrap()),
            cpu_arch: System::cpu_arch().unwrap_or("UNKNOWN".parse().unwrap()),
            kernel_version: System::kernel_version().unwrap_or("UNKNOWN".parse().unwrap()),
            distribution_id: System::distribution_id(),
        }
    }
    pub fn to_context(&self) -> tera::Context {
        let mut con = Context::new();
        con.insert("system_name", &self.system_name);
        con.insert("hostname", &self.host_name);
        con.insert("os_version", &self.os_version);
        con.insert("cpu_arch", &self.cpu_arch);
        con.insert("kernel_version", &self.kernel_version);
        con.insert("distribution_id", &self.distribution_id);
        con
    }
}

#[handler]
pub async fn get_gross_info(res: &mut Response) {
    let gross = Gross::new();
    res.render(Json(gross));
}
