use sysinfo::System;

fn main() {
    println!("Starting System Information Grab...");
    grab_memory();
}

fn grab_memory() {
    let mut sys = System::new_all();
    sys.refresh_memory();

    println!("System Memory Info: ");
    println!("  Total Memory: {} bytes", sys.total_memory());
    println!("  Memory In Use: {} bytes", sys.used_memory());
    println!("  Total Swap: {} bytes", sys.total_swap());
    println!("  Swap In Use: {} bytes", sys.used_swap());
}
