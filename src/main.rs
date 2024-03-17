use core::time;

use sysinfo::System;

fn main() {
    println!("Starting System Information Grab...");
    grab_memory();
    name_version(true);
    grab_cpu_data();
}

fn name_version(long_name: bool) {
    println!("System Identity: ");
    println!("  System Name: {:?}", System::name());
    println!("  Kernel Version {:?}", System::kernel_version());
    if long_name == true {println!("  OS Version {:?}", System::long_os_version());}
    else {println!("  OS Version {:?}", System::os_version());}
    println!("  Host Name {:?}", System::host_name());
}

fn simplify_data(data_size: u64) -> (u64, String){
    let mut exponent: u32 = 0;
    let mut number = data_size;
    let mut amount: &str = "Null";
    while  number > 1024 {
        number =  number / 1024;
        exponent += 1;
    }
    
    match exponent{
        0=>{amount = "B"},
        1=>{amount = "KB"},
        2=>{amount = "MB"},
        3=>{amount = "GB"},
        4=>{amount = "TB"},
        5=>{amount = "PB"},
        6=>{amount = "EB"},
        7=>{amount = "ZB"},
        _=>{amount = "Error: Size Unknown"}
    }

    return (number, amount.to_string());
}

fn grab_memory() {
    let mut sys = System::new();
    sys.refresh_memory();
    let tmi = simplify_data(sys.total_memory());
    let miu = simplify_data(sys.used_memory());
    let tsi = simplify_data(sys.total_swap());
    let siu = simplify_data(sys.used_swap());
    println!("System Memory Info: ");
    println!("  Total Memory: {} {}", tmi.0, tmi.1);
    println!("  Memory In Use: {} {}", miu.0, miu.1); 
    println!("  Total Swap: {} {}", tsi.0, tsi.1);
    println!("  Swap In Use: {} {}", siu.0, siu.1);
}

fn simplify_frequency(freq: u64) -> (u64, String) {
    let mut exponent = 0;
    let mut simple_freq = freq;
    let mut amount: &str = "Null";

    while simple_freq > 1000 {
        simple_freq = simple_freq / 1000;
        exponent += 1;
    }

    match exponent{
        0=>{amount = "Hz"},
        1=>{amount = "KHz"},
        2=>{amount = "MHz"},
        3=>{amount = "GHz"},
        4=>{amount = "THz"},
        _=>{amount = "Error: Freq Unknown"}
    }

    return (simple_freq, amount.to_string());
}

fn grab_cpu_data() {
    let mut sys = System::new();
    sys.refresh_cpu();
    let sleep_time = time::Duration::from_millis(200);
    let now = std::time::Instant::now();
    std::thread::sleep(sleep_time);
    assert!(now.elapsed() >= sleep_time);
    sys.refresh_cpu();
    let cpu_info = sys.global_cpu_info();
    let freq_info = simplify_frequency(cpu_info.frequency());
    println!("Cpu Info: ");
    println!("  CPU Count: {}", sys.cpus().len());
    println!("  CPU Vendor ID {}", cpu_info.vendor_id());
    println!("  CPU Brand {}", cpu_info.brand());
    println!("  CPU Name {}", cpu_info.name());
    println!("  CPU Frequency {} {}", freq_info.0, freq_info.1);
}