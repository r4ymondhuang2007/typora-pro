//! Windows registry read helpers for Typora Pro.

pub const APP_KEY: &str = "Software\\Typora\\Typora Pro";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Typora Pro";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Typora Pro") },
        RegValue { name: "Version",     value: String::from("v1.5.25") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Typora Pro") },
        RegValue { name: "Publisher",       value: String::from("Typora") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Typora Pro\\uninstall.exe\"") },
    ]
}
