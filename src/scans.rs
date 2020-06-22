/// This file holds logic related to UTXO-set scans
use crate::node_interface::{register_scan};
use json;

/// Saves UTXO-set scan ids to scanIDs.json
pub fn save_scan_ids_locally(epoch_preparation_id: String, pool_epoch_id: String, datapoint_id: String, pool_deposit_id: String) {
    let id_json = object!{
        epoch_preparation_scan_id: epoch_preparation_id,
        oracle_pool_epoch_scan_id: pool_epoch_id,
        datapoint_scan_id: datapoint_id,
        pool_deposit_scan_id: pool_deposit_id,
    };
    std::fs::write("scanIDs.json", json::stringify_pretty(id_json, 4)).expect("Unable to save UTXO-set scan ids to scanIDs.json");
}


/// This function registers scanning for the Epoch Preparation stage box
pub fn register_epoch_preparation_scan(oracle_pool_nft: &String, epoch_preparation_address: &String) -> String {
    // Scan for NFT id + Epoch Preparation address
    let scan_json = object!{
        appName: "Epoch Preparation Scan",
        trackingRule: {
            "predicate": "and",
            "args": [
                {
                "predicate": "containsAsset",
                "assetId": oracle_pool_nft.clone(),
                },
                {
                "predicate": "equals",
                "bytes": epoch_preparation_address.clone(),
                }
            ]}
        };

    register_scan(&json::stringify(scan_json.clone())).expect("Failed to register epoch preparation scan.")
}


/// This function registers scanning for the Oracle Pool Epoch stage box
pub fn register_oracle_pool_epoch_scan(oracle_pool_nft: &String, pool_epoch_address: &String) -> String {
    // Scan for NFT id + Oracle Pool Epoch address
    let scan_json = object!{
        appName: "Oracle Pool Epoch Scan",
        trackingRule: {
            "predicate": "and",
            "args": [
                {
                "predicate": "containsAsset",
                "assetId": oracle_pool_nft.clone(),
                },
                {
                "predicate": "equals",
                "bytes": pool_epoch_address.clone(),
                }
            ]}
        };

    register_scan(&json::stringify(scan_json.clone())).expect("Failed to register oracle pool epoch scan.")
}

/// This function registers scanning for the oracle's personal Datapoint box
pub fn register_datapoint_scan(oracle_pool_participant_token: &String, datapoint_address: &String, oracle_address: &String) -> String {
    // Scan for pool participant token id + oracle_address in R4
    let scan_json = object!{
        appName: "Personal Oracle Datapoint Scan",
        trackingRule: {
            "predicate": "and",
            "args": [
                {
                "predicate": "containsAsset",
                "assetId": oracle_pool_participant_token.clone(),
                },
                {
                "predicate": "equals",
                "bytes": datapoint_address.clone(),
                },
                {
                "predicate": "equals",
                "register": "R4",
                "bytes": oracle_address.clone(),
                }
            ]}
        };

    register_scan(&json::stringify(scan_json.clone())).expect("Failed to register oracle datapoint scan.")
}

/// This function registers scanning for any boxes in the Pool Deposit stage address
pub fn register_pool_deposit_scan(pool_deposit_address: &String) -> String {
    // Scan for boxes at pool deposit address
    let scan_json = object!{
        appName: "Oracle Pool Deposit Scan",
        trackingRule: {
                "predicate": "equals",
                "bytes": pool_deposit_address.clone(),
        }
    };

    register_scan(&json::stringify(scan_json.clone())).expect("Failed to register pool deposit scan.")
}