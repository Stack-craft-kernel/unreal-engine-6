//! Windows registry read helpers for Unreal Engine 6.

pub const APP_KEY: &str = "Software\\Unreal\\Unreal Engine 6";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Unreal Engine 6";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Unreal Engine 6") },
        RegValue { name: "Version",     value: String::from("v1.0.22") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Unreal Engine 6") },
        RegValue { name: "Publisher",       value: String::from("Unreal") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Unreal Engine 6\\uninstall.exe\"") },
    ]
}
