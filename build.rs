const COMMANDS: &[&str] = &[
    "connect",
    "connect_with_bind",
    "disconnect",
    "send",
    "is_connected",
    "get_connections",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
