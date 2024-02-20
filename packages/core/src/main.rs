#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

fn main() {
    discord::connect();

    #[cfg(target_os = "macos")]
    macos::main();
    #[cfg(target_os = "windows")]
    window::main();
}
