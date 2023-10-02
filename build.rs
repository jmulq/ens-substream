use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("BaseRegistrarImplementation", "abi/base_registrar.json")?
        .generate()?
        .write_to_file("src/abi/base_registrar.rs")?;

    Abigen::new(
        "EthRegistrarController",
        "abi/eth_registrar_controller.json",
    )?
    .generate()?
    .write_to_file("src/abi/eth_registrar_controller.rs")?;

    Ok(())
}
