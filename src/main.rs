mod config;
mod logger;
mod core;
mod os;

use crate::core::jexus::Jexus;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Jexus::init();
    Ok(())
}
