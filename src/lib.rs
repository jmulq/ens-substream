mod abi;
mod constants;
mod helpers;
mod pb;

use abi::{base_registrar, eth_registrar_controller};
use helpers::{create_event_id, name_hash};
use pb::eth::ens::v1 as ens;
use substreams::Hex;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

#[substreams::handlers::map]
fn map_domain(block: eth::Block) -> Result<Option<ens::Domains>, substreams::errors::Error> {
    let domains: Vec<_> = block
        .events::<eth_registrar_controller::events::NameRegistered>(&[
            &constants::ETH_REG_CONTROLLER,
        ])
        .map(|(event, _log)| {
            let name = event.name.clone() + ".eth";
            ens::Domain {
                name,
                label_name: event.name,
                label_hash: Hex(event.label.to_vec()).to_string(),
            }
        })
        .collect();

    if domains.is_empty() {
        return Ok(None);
    } else {
        Ok(Some(ens::Domains { domains }))
    }
}

#[substreams::handlers::map]
fn map_transfer(
    block: eth::Block,
) -> Result<Option<ens::NameTransfers>, substreams::errors::Error> {
    let name_transfers: Vec<_> = block
        .events::<base_registrar::events::Transfer>(&[&constants::BASE_REGISTRAR])
        .map(|(event, log)| ens::NameTransfer {
            from: Some(ens::Account {
                address: helpers::format_hex(&event.from),
            }),
            to: Some(ens::Account {
                address: helpers::format_hex(&event.to),
            }),
            token_id: event.token_id.to_string(),
            block_number: block.number,
            tx_hash: log.receipt.transaction.hash.clone(),
            log_index: log.block_index(),
        })
        .collect();

    if name_transfers.is_empty() {
        return Ok(None);
    } else {
        Ok(Some(ens::NameTransfers { name_transfers }))
    }
}

#[substreams::handlers::map]
pub fn graph_out(
    domains: ens::Domains,
    name_transfers: ens::NameTransfers,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for domain in domains.domains.into_iter() {
        tables
            .create_row("Domain", name_hash(&domain.name).to_string())
            .set("name", domain.name)
            .set("labelName", domain.label_name)
            .set("labelHash", domain.label_hash);
    }

    for transfer in name_transfers.name_transfers.into_iter() {
        if let Some(to) = &transfer.to {
            tables
                .create_row(
                    "NameTransferred",
                    create_event_id(&transfer.block_number, &transfer.log_index),
                )
                .set("tokenID", transfer.token_id)
                .set("blockNumber", transfer.block_number)
                .set("transactionID", transfer.tx_hash)
                .set("owner", &to.address);
        }
    }

    Ok(tables.to_entity_changes())
}
