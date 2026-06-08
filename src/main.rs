mod collectors;
pub mod password;
mod s3;
mod structs;

use std::io;
use crate::collectors::*;
use crate::s3::*;
use crate::structs::*;

use sysinfo::System;

// --- Builder ---

fn build_machine_profile() -> MachineProfile {
    let mut sys = System::new_all();
    sys.refresh_all();

    MachineProfile {
        system: collect_system_info(&sys),
        location_and_purpose: collect_location_and_purpose(),
        disks: collect_disk_info(),
        networks: collect_network_info(),
        users: collect_users(),
        apps: collect_apps(),
        work_school_accounts: collect_work_school_accounts(),
        microsoft_accounts: collect_microsoft_accounts(),
    }
}

// --- Run Binary Loop ---

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile = build_machine_profile();
    println!("Built machine profile for {} with mac address {}", profile.system.host_name, profile
        .networks.first().unwrap().mac_address);
    write_to_s3(&profile, &profile.system.host_name).await?;

    println!("Press Enter key to exit");
    io::stdin().read_line(&mut String::new())?;

    Ok(())
}
