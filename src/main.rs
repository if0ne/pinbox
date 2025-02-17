mod category;
mod cmd;
mod config;
mod errors;
mod pin;
mod source;

pub const APP_NAME: &str = "pinbox";

fn main() {
    cmd::Cmd::run();
}
