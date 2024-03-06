use async_std::fs;
use futures::future::join_all;
use namada_light_sdk::namada_sdk::eth_bridge::ethers::abi::token;
use std::collections::BTreeMap;
use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::str::FromStr;
use std::{thread, time, vec};


use namada_light_sdk::transaction::transfer::Transfer;
use namada_light_sdk::namada_sdk::NamadaImpl; // This module allows us to access the NamadaImpl struct, which is needed for most transactions

use namada_light_sdk::namada_sdk::wallet::Wallet;
use namada_light_sdk::namada_sdk::chain::ChainId;


use namada_light_sdk::namada_sdk::hash::Hash;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Define constants
    let CHAIN_ID = "shielded-expedition.88f17d1d14";
    let TENDERMIND_ADDR = "http://localhost:26657";

    // Create source and target addresses
    let source_address = namada_light_sdk::namada_sdk::address::Address::from_str("tnam1v4ehgw36xq6ngs3ng5crvdpngg6yvsecx4znjdfegyurgwzzx4pyywfexuuyys69gc6rzdfnryrntx").unwrap();
    let target_address = source_address.clone(); // Using the same address for simplicity

    // Create secret key
    let secret_key = namada_light_sdk::namada_sdk::key::common::SecretKey::from_str("c3b").unwrap();

    // Query the native token address
    let token_address = namada_light_sdk::reading::asynchronous::query_native_token(TENDERMIND_ADDR).await.unwrap();

    // Define the amount to transfer
    let amount = namada_light_sdk::namada_sdk::token::DenominatedAmount::from_str("10000").unwrap();

    // Define global arguments for the transaction
    let global_args = namada_light_sdk::transaction::GlobalArgs{
        expiration: None,
        code_hash: Hash::from_str("a78ae102c6fa12c9021328335f6cc7a38d33f67855e297f42b2095990a4b42fa").unwrap(),
        chain_id: ChainId::from_str(CHAIN_ID).unwrap(),
    };

    // Create a new transfer
    let mut transfer = Transfer::new(
        source_address,
        target_address,
        token_address.clone(),
        amount,
        None,
        None,
        global_args
    );

    // Define targets and secret keys for the signature
    let targets = vec![transfer.clone().payload().raw_header_hash()];
    let mut secret_keys = BTreeMap::new();
    secret_keys.insert(0, secret_key.clone());

    // Create a new signature
    let signature_tx = namada_light_sdk::namada_sdk::tx::Signature::new(targets.clone(), secret_keys.clone(), None);
    let signature = signature_tx.signatures.get(&0u8).unwrap().to_owned();

    // Attach the signature to the transfer
    transfer = transfer.attach_signatures(secret_key.clone().to_public(), signature.clone());

    // Define fee and gas limit
    let fee = namada_light_sdk::namada_sdk::token::DenominatedAmount::from_str("10").unwrap();
    let gas_limit = namada_light_sdk::namada_sdk::tx::data::GasLimit::from_str("20000").unwrap();

    // Define epoch
    let epoch = namada_light_sdk::namada_sdk::proof_of_stake::Epoch::from_str("0").unwrap();

    // Attach the fee to the transfer
    transfer = transfer.attach_fee(fee, token_address.clone(), secret_key.clone().to_public(), epoch, gas_limit);

    // Create a new fee signature
    let fee_signature_tx = namada_light_sdk::namada_sdk::tx::Signature::new(targets, secret_keys.clone(), None);
    let fee_signature = fee_signature_tx.signatures.get(&0u8).unwrap().to_owned();

    // Attach the fee signature to the transfer
    transfer = transfer.attach_fee_signature(secret_key.clone().to_public(), signature.clone());

    // Get the payload of the transfer
    let transfer_tx = transfer.payload();

    // Broadcast the transaction
    let response = namada_light_sdk::writing::asynchronous::broadcast_tx(TENDERMIND_ADDR, transfer_tx);

    Ok(())
}
//     // Setup client
