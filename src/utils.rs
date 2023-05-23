use std::env;

pub(crate) fn vale_arch() -> String {
    let platform = match env::consts::OS {
        "windows" => "Windows",
        "macos" => "macOS",
        _ => "Linux",
    };
    let arch = match env::consts::ARCH {
        "x86_64" => "64-bit",
        "arm" => "arm64",
        "aarch64" => "arm64",
        _ => "386",
    };
    format!("{}_{}", platform, arch)
}
