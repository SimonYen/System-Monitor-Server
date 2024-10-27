use crate::DISKS;
use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct Disks {
    name: Vec<String>,         //硬盘名
    file_system: Vec<String>,  //文件系统
    mount_point: Vec<String>,  //挂载点
    available_space: Vec<u64>, //可用空间
    total_space: Vec<u64>,     //总空间
}

impl Disks {
    fn new() -> Self {
        let mut disks = DISKS.lock().unwrap();
        //刷新
        disks.refresh_list();
        //构建数组
        let mut vec_name: Vec<String> = Vec::new();
        let mut vec_file_system: Vec<String> = Vec::new();
        let mut vec_mount_point: Vec<String> = Vec::new();
        let mut vec_available_space: Vec<u64> = Vec::new();
        let mut vec_total_space: Vec<u64> = Vec::new();
        //遍历
        for disk in disks.list() {
            vec_name.push(disk.name().to_str().unwrap_or("UNKNOWN").to_string());
            vec_file_system.push(disk.file_system().to_str().unwrap_or("UNKNOWN").to_string());
            vec_mount_point.push(disk.mount_point().to_str().unwrap_or("UNKNOWN").to_string());
            vec_available_space.push(disk.available_space());
            vec_total_space.push(disk.total_space());
        }
        Disks {
            name: vec_name,
            file_system: vec_file_system,
            mount_point: vec_mount_point,
            available_space: vec_available_space,
            total_space: vec_total_space,
        }
    }
}

#[handler]
pub async fn get_all_info(res: &mut Response) {
    let disks=Disks::new();
    res.render(Json(disks));
}