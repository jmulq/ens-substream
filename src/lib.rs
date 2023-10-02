mod abi;
mod pb;
mod constants;
mod helpers;

use helpers::{name_hash, create_event_id};
use pb::eth::ens::v1 as ens;
use substreams::Hex;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

#[substreams::handlers::map]
fn map_domain(block: eth::Block) -> Result<Option<ens::Domains>, substreams::errors::Error> {
    let domains: Vec<_> = block
        .events::<abi::eth_registrar_controller::events::NameRegistered>(&[&constants::ETH_REG_CONTROLLER])
        .map(|(event, _log)| {
            substreams::log::info!("ENS Domain Registered");
            let name = event.name.clone() + ".eth";

            ens::Domain {
                name,
                label_name: event.name,
                label_hash: Hex(event.label.to_vec()).to_string(),
            }
        })
        .collect();

    if domains.len() == 0 {
        return Ok(None);
    }
    Ok(Some(ens::Domains { domains }))
}

#[substreams::handlers::map]
fn map_transfer(block: eth::Block) -> Result<Option<ens::NameTransfers>, substreams::errors::Error> {
    let name_transfers: Vec<_> = block
        .events::<abi::base_registrar::events::Transfer>(&[&constants::BASE_REGISTRAR])
        .map(|(event, log)| {
            substreams::log::info!("ENS Domain Transfer");

            ens::NameTransfer {
                from: Some(ens::Account {
                    address: helpers::format_hex(&event.from),
                }),
                to: Some(ens::Account {
                    address: helpers::format_hex(&event.to)
                }),
                token_id: event.token_id.to_string(),
                block_number: block.number,
                tx_hash: helpers::format_hex(&log.receipt.transaction.hash),
                log_index: log.block_index()
            }
        }).collect();

    if name_transfers.len() == 0 {
        return Ok(None);
    }
    Ok(Some(ens::NameTransfers { name_transfers }))
}

#[substreams::handlers::map]
pub fn graph_out(domains: ens::Domains, name_transfers: ens::NameTransfers) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for domain in domains.domains.into_iter() {
        tables
            .create_row("Domain", name_hash(&domain.name).to_string())
            .set("name", domain.name)
            .set("labelName", domain.label_name)
            .set("labelHash", domain.label_hash);
    }

    for transfer in name_transfers.name_transfers.into_iter() {        
        tables
            .create_row("Transfer", create_event_id(&transfer.block_number, &transfer.log_index))
            .set("tokenID", transfer.token_id)
            .set("blockNumber", transfer.block_number)
            .set("transactionID", transfer.tx_hash)
            .set("owner", &transfer.to.as_ref().unwrap().address);
    }

    Ok(tables.to_entity_changes())
}