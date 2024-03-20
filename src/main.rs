use core::time;
use inline_colorization::*;
use sysinfo::{Networks, System};

/*
    Colors & their meanings:
        Blue: Component type stuff like memory or the physical hardware or the CPU.
        Red: Network and system identity information.
        Yellow: Disk information
        Cyan: Processes
        Underlined Bright Green: number associated with some form of information
        Underlined Bright Blue: IP/MAC Address
*/

fn main() {
    println!("Starting System Information Grab...");
    name_version(true);
    get_networks();
    grab_memory();
    grab_cpu_data();
    get_disks();
    get_process(10);
}

fn name_version(long_name: bool) {
    println!("{color_red}System Identity: {color_reset}");
    println!("    System Name: {:?}", System::name());
    println!("    Kernel Version {:?}", System::kernel_version());
    if long_name == true {println!("    OS Version {:?}", System::long_os_version());}
    else {println!("    OS Version {:?}", System::os_version());}
    println!("    Host Name {:?}", System::host_name());
    println!(" ");
}

fn simplify_data(data_size: u64) -> (f64, String){
    let mut exponent: f64 = 0.0;
    let mut exponent_int: u32 = 0;
    let mut number = data_size;
    let data_size_float = data_size as f64;
    let mut amount: &str = "Null";

    while  number > 1024 {
        number =  number / 1024; 
        exponent += 1.0;
        exponent_int += 1;
    }

    let decimals = ((data_size_float)/(1024.0_f64.powf(exponent - 1.0)))/1024.0;
    
    match exponent_int {
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

    return ((((decimals * 100.0).round())/100.0), amount.to_string());
}

fn grab_memory() {
    let mut sys = System::new();
    sys.refresh_memory();
    let tmi = simplify_data(sys.total_memory());
    let miu = simplify_data(sys.used_memory());
    let tsi = simplify_data(sys.total_swap());
    let siu = simplify_data(sys.used_swap());
    let fm= simplify_data(sys.total_memory() - sys.used_memory());
    let fs= simplify_data(sys.total_swap() - sys.used_swap());
    println!("{color_blue}System Memory Info: {color_reset}");
    println!("    Total Memory: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
    Memory Free: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
    Memory In Use: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
    Total Swap: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
    Free Swap: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
    Swap In Use: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
    ", tmi.0, tmi.1, fm.0, fm.1, miu.0, miu.1, tsi.0, tsi.1, fs.0, fs.1, siu.0, siu.1);
}

fn simplify_frequency(freq: u64) -> (u64, String) {
    let mut exponent = 0;
    let mut simple_freq = freq;
    let mut amount: &str = "Null";

    while simple_freq > 1000 {
        simple_freq = simple_freq / 1000;
        exponent += 1;
    }

    match exponent {
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
    println!("{color_blue}Cpu Info: {color_reset}");
    println!("    CPU Count: {}
    CPU Vendor ID {}
    CPU Brand {}
    CPU Name {}
    CPU Frequency {} {}
    ", sys.cpus().len(), cpu_info.vendor_id(), cpu_info.brand(), cpu_info.name(), freq_info.0, freq_info.1);
}

fn get_disks() {
    let disks = sysinfo::Disks::new_with_refreshed_list();
    println!("{color_yellow}Disks: {color_reset}");
    for disk in &disks {
        let a = simplify_data(disk.available_space());
        let ta = simplify_data(disk.total_space());
        println!("  {color_yellow}{style_underline}Disk{style_reset}{color_reset} {color_yellow}{style_underline}{:?}{style_reset}{color_reset}:
        File System: {:?}
        Removable: {:?}
        Mounted @: {:?}
        Type: {:?}
        Available Space {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
        Total Space {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
        ", disk.name(), disk.file_system(), disk.is_removable(), disk.mount_point(), disk.kind(), a.0, a.1, ta.0, ta.1);
    }
}

fn get_networks() {
    let networks = Networks::new_with_refreshed_list();
    println!("{color_red}Networks:{color_reset}");

    for (interface_name, data) in &networks {
        let simplified_recieved = simplify_data(data.received());
        let simplified_sent = simplify_data(data.transmitted());
        let simplified_tsent = simplify_data(data.total_transmitted());
        let simplified_trecieved = simplify_data(data.total_received());
        println!("   {color_red}{style_underline}{interface_name}{style_reset}{color_reset}:
        MAC Address: {color_bright_blue}{style_underline}{}{style_reset}{color_reset}
        Sent/Recieved: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {} / {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
        Total Sent/Recieved: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {} / {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
        Packet Total Sent/Recieved: {color_bright_green}{style_underline}{}{style_reset}{color_reset} / {color_bright_green}{style_underline}{}{style_reset}{color_reset}
            ",data.mac_address(), simplified_sent.0, simplified_sent.1, simplified_recieved.0, simplified_recieved.1, simplified_tsent.0, simplified_tsent.1, simplified_trecieved.0, simplified_trecieved.1, data.total_packets_transmitted(), data.total_packets_received());
    }
}

fn get_process(amount: i32) {
    let mut sys = System::new();
    let mut iterator = 1;
    sys.refresh_all();

    println!("{color_cyan}PID & Processes: {color_reset}");

    for (pid, process) in sys.processes() {
        if iterator <= amount {
            let vmem = simplify_data(process.memory());
            println!("  {color_cyan}{style_underline}[{pid}]{style_reset}{color_reset} {color_cyan}{style_underline}{}{style_reset}{color_reset}:
            CPU Usage: {color_bright_green}{style_underline}{:?}{style_reset}{color_reset}
            Memory Usage: {color_bright_green}{style_underline}{}{style_reset}{color_reset} {}
            Status: {:?}
            ", process.name(), process.cpu_usage(), vmem.0, vmem.1, process.status());
            iterator += 1;
        }
    }
}