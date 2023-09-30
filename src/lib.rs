mod abi;
mod pb;
mod constants;
mod helpers;

use pb::eth::ens::v1 as ens;
use substreams::Hex;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

#[substreams::handlers::map]
fn map_domains(block: eth::Block) -> Result<Option<ens::Domains>, substreams::errors::Error> {
    let domains: Vec<_> = block
        .events::<abi::eth_registrar_controller::events::NameRegistered>(&[&constants::ETH_REG_CONTROLLER])
        .map(|(event, _log)| {
            substreams::log::info!("ENS Domain Registered");
            let name = event.name.clone() + ".eth";

            ens::Domain {
                name,
                label_name: event.name,
                label_hash: Hex(event.label.to_vec()).to_string(),
                owner: Some(ens::Account {
                    address: helpers::format_hex(&event.owner)
                })
            }
        })
        .collect();

    if domains.len() == 0 {
        return Ok(None);
    }
    Ok(Some(ens::Domains { domains }))
}

// TODO: How can we update the domains entities via these events.
#[substreams::handlers::map]
fn map_transfers(block: eth::Block) -> Result<Option<ens::Transfers>, substreams::errors::Error> {
    let transfers: Vec<_> = block
        .events::<abi::base_registrar::events::Transfer>(&[&constants::BASE_REGISTRAR])
        .map(|(event, log)| {
            substreams::log::info!("ENS Domain Transfer");

            ens::Transfer {
                from: Some(ens::Account {
                    address: helpers::format_hex(&event.from),
                }),
                to: Some(ens::Account {
                    address: helpers::format_hex(&event.to)
                }),
                token_id: event.token_id.to_string(),
                block_number: block.number,
                tx_hash: helpers::format_hex(&log.receipt.transaction.hash)
            }
        }).collect();

    if transfers.len() == 0 {
        return Ok(None);
    }
    Ok(Some(ens::Transfers { transfers }))
}
