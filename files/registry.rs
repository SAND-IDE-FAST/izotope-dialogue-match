//! Windows registry read helpers for Izotope Dialogue Match.

pub const APP_KEY: &str = "Software\\Izotope\\Izotope Dialogue Match";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Izotope Dialogue Match";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Izotope Dialogue Match") },
        RegValue { name: "Version",     value: String::from("v4.7.13") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Izotope Dialogue Match") },
        RegValue { name: "Publisher",       value: String::from("Izotope") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Izotope Dialogue Match\\uninstall.exe\"") },
    ]
}
