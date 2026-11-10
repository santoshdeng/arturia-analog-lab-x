//! Windows registry read helpers for Arturia Analog Lab X.

pub const APP_KEY: &str = "Software\\Arturia\\Arturia Analog Lab X";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Arturia Analog Lab X";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Arturia Analog Lab X") },
        RegValue { name: "Version",     value: String::from("v3.6.12") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Arturia Analog Lab X") },
        RegValue { name: "Publisher",       value: String::from("Arturia") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Arturia Analog Lab X\\uninstall.exe\"") },
    ]
}
