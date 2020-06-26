#[macro_use]
extern crate json;

mod actions;
mod oracle_config;
mod oracle_state;
mod node_interface;
mod scans;


pub type NanoErg = u64;
pub type BlockHeight = u64;
/// The id of the oracle pool epoch box
pub type EpochID = String;




fn main() {
    println!("Hello, oracle pool!");

    let node_url = oracle_config::get_node_url();
    let node_api_key = oracle_config::get_node_api_key();

    let op = oracle_state::OraclePool::new();

    let boxes = node_interface::get_scan_boxes(&op.pool_deposit_stage.scan_id);

    node_interface::current_block_height();
}



