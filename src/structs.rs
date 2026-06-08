use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub host_name: String,
    pub os_version: String,
    pub windows_edition: String,
    pub kernel_version: String,
    pub total_memory_mb: u64,
    pub cpu_vendor_id: String,
    pub cpu_brand: String,
    pub cpu_frequency: u64,
}

#[derive(Debug, Serialize)]
pub struct DiskInfo {
    pub index: usize,
    pub total_space_gb: u64,
}

#[derive(Debug, Serialize)]
pub struct NetworkInfo {
    pub interface_name: String,
    pub mac_address: String,
    pub ip_networks: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub groups: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct WorkSchoolAccount {
    pub thumbprint: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MicrosoftAccount {
    pub account_id: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LocationAndPurpose {
    pub purpose: String,
    pub location: String,
}

#[derive(Debug, Serialize)]
pub struct MachineProfile {
    pub system: SystemInfo,
    pub location_and_purpose: LocationAndPurpose,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetworkInfo>,
    pub users: Vec<UserInfo>,
    pub work_school_accounts: Vec<WorkSchoolAccount>,
    pub microsoft_accounts: Vec<MicrosoftAccount>,
    pub apps: Vec<AppInfo>,
}
