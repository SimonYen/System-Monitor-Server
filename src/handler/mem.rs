use crate::SYSTEM;
use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct PhysicalMemory {
    used: u64,  //已使用的物理内存量
    free: u64,  //空闲物理内存量
    total: u64, //总共物理内存量
}

#[derive(Serialize)]
struct SwapMemory {
    used: u64,  //已使用的交换内存量
    free: u64,  //可用交换内存量
    total: u64, //总共交换内存量
}

#[derive(Serialize)]
struct Memory {
    physical_memory: PhysicalMemory,
    swap_memory: SwapMemory,
}

impl Memory {
    fn new() -> Self {
        let mut sys = SYSTEM.lock().unwrap();
        //刷新
        sys.refresh_memory();
        Memory {
            physical_memory: PhysicalMemory {
                used: sys.used_memory(),
                free: sys.free_memory(),
                total: sys.total_memory(),
            },
            swap_memory: SwapMemory {
                used: sys.used_swap(),
                free: sys.free_swap(),
                total: sys.total_swap(),
            },
        }
    }
}

#[handler]
pub async fn get_all_info(res: &mut Response) {
    let mem = Memory::new();
    res.render(Json(mem));
}
