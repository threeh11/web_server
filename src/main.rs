mod config;
mod core;
mod os;

use core::jexus::Jexus;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Jexus::init();

    Ok(())
}
