use crate::structs::*;
use installed::list;
use sysinfo::{Disks, Networks, System, Users};
use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ};
use std::io::{self, Write};

pub fn collect_system_info(sys: &System) -> SystemInfo {
    // Safely extract the first CPU without triggering runtime panic strings
    let first_cpu = System::cpus(sys).first();

    SystemInfo {
        host_name: System::host_name().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        windows_edition: collect_windows_edition().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        total_memory_mb: sys.total_memory() / 1_000_000,
        cpu_vendor_id: first_cpu
            .map(|c| c.vendor_id().to_string())
            .unwrap_or_default(),
        cpu_brand: first_cpu.map(|c| c.brand().to_string()).unwrap_or_default(),
        cpu_frequency: first_cpu.map(|c| c.frequency()).unwrap_or(0),
    }
}

pub fn collect_location_and_purpose() -> LocationAndPurpose {
    print!("Where is this computer? ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut location = String::new();
    io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");

    print!("What is the main use of this computer? ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut purpose = String::new();
    io::stdin()
        .read_line(&mut purpose)
        .expect("Failed to read line");
    LocationAndPurpose {
        location,
        purpose,
    }
}

pub fn collect_disk_info() -> Vec<DiskInfo> {
    Disks::new_with_refreshed_list()
        .iter()
        .enumerate()
        .map(|(i, disk)| DiskInfo {
            index: i,
            total_space_gb: disk.total_space() / 1_000_000_000,
        })
        .collect()
}

pub fn collect_network_info() -> Vec<NetworkInfo> {
    Networks::new_with_refreshed_list()
        .iter()
        .map(|(name, data)| NetworkInfo {
            interface_name: name.clone(),
            mac_address: data.mac_address().to_string(),
            ip_networks: data
                .ip_networks()
                .iter()
                .map(|ip| format!("{}/{}", ip.addr, ip.prefix))
                .collect(),
        })
        .collect()
}

pub fn collect_windows_edition() -> Option<String> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .and_then(|key| key.get_value("EditionID"))
        .ok()
}

pub fn collect_users() -> Vec<UserInfo> {
    Users::new_with_refreshed_list()
        .list()
        .iter()
        .map(|user| UserInfo {
            name: user.name().to_string(),
            groups: user.groups().iter().map(|g| g.name().to_string()).collect(),
        })
        .collect()
}

pub fn collect_apps() -> Vec<AppInfo> {
    let apps = match list() {
        Ok(iter) => iter.collect::<Vec<_>>(),
        Err(_) => return vec![],
    };

    apps.into_iter()
        .filter(|app| !app.name().is_empty())
        .map(|app| AppInfo {
            name: app.name().to_string(),
            version: app.version().to_string(),
        })
        .collect()
}

pub fn collect_work_school_accounts() -> Vec<WorkSchoolAccount> {
    const JOIN_PATH: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\WorkplaceJoin\JoinInfo";

    let reg_key =
        match RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(JOIN_PATH, KEY_READ) {
            Ok(k) => k,
            Err(_) => return vec![],
        };

    reg_key
        .enum_keys()
        .filter_map(Result::ok)
        .map(|thumbprint| {
            let email = reg_key
                .open_subkey_with_flags(&thumbprint, KEY_READ)
                .ok()
                .and_then(|sub| sub.get_value::<String, _>("UserEmail").ok());

            WorkSchoolAccount { thumbprint, email }
        })
        .collect()
}

pub fn collect_microsoft_accounts() -> Vec<MicrosoftAccount> {
    const IDENTITY_PATH: &str = r"Software\Microsoft\IdentityCRL\UserExtendedProperties";

    // Open the primary IdentityCRL key using identical flag structures
    let reg_key =
        match RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(IDENTITY_PATH, KEY_READ) {
            Ok(k) => k,
            Err(_) => return vec![], // Return an empty vector if the key doesn't exist
        };

    reg_key
        .enum_keys()
        .filter_map(Result::ok) // Safely skip any unreadable subkeys
        .map(|account_id| {
            // Open each unique account subkey to extract its matching email value
            let email = reg_key
                .open_subkey_with_flags(&account_id, KEY_READ)
                .ok()
                .and_then(|sub| sub.get_value::<String, _>("EmailAddress").ok());

            MicrosoftAccount { account_id, email }
        })
        .collect()
}
