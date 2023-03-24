use crate::box_kind::{
    BallotBoxError, BallotBoxWrapper, BallotBoxWrapperInputs, OracleBox, OracleBoxError,
    OracleBoxWrapper, OracleBoxWrapperInputs, PoolBox, PoolBoxError, PoolBoxWrapper,
    PoolBoxWrapperInputs, PostedOracleBox, RefreshBoxError, RefreshBoxWrapper,
    RefreshBoxWrapperInputs, UpdateBoxError, UpdateBoxWrapper, UpdateBoxWrapperInputs,
    VoteBallotBoxWrapper,
};
use crate::datapoint_source::DataPointSourceError;
use crate::oracle_config::ORACLE_CONFIG;
use crate::oracle_types::{BlockHeight, EpochCounter};
use crate::pool_config::POOL_CONFIG;
use crate::scans::{load_scan_ids, OracleTokenScan, Scan, ScanError, ScanGetBoxes};
use anyhow::Error;
use derive_more::From;

use ergo_lib::ergotree_ir::chain::address::Address;
use ergo_lib::ergotree_ir::mir::constant::TryExtractFromError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DataSourceError>;

#[derive(Debug, From, Error)]
pub enum DataSourceError {
    #[error("unexpected data error: {0}")]
    UnexpectedData(TryExtractFromError),
    #[error("scan error: {0}")]
    ScanError(ScanError),
    #[error("pool box error: {0}")]
    PoolBoxError(PoolBoxError),
    #[error("pool box not found")]
    PoolBoxNotFoundError,
    #[error("ballot box error: {0}")]
    BallotBoxError(BallotBoxError),
    #[error("refresh box error: {0}")]
    RefreshBoxError(RefreshBoxError),
    #[error("refresh box not found")]
    RefreshBoxNotFoundError,
    #[error("oracle box error: {0}")]
    OracleBoxError(OracleBoxError),
    #[error("datapoint source error: {0}")]
    DataPointSource(DataPointSourceError),
    #[error("update box error: {0}")]
    UpdateBoxError(UpdateBoxError),
    #[error("update box not found")]
    UpdateBoxNotFoundError,
}

pub trait PoolBoxSource {
    fn get_pool_box(&self) -> Result<PoolBoxWrapper>;
}

pub trait LocalBallotBoxSource {
    fn get_ballot_box(&self) -> Result<Option<BallotBoxWrapper>>;
}

pub trait RefreshBoxSource {
    fn get_refresh_box(&self) -> Result<RefreshBoxWrapper>;
}

pub trait DatapointBoxesSource {
    fn get_oracle_datapoint_boxes(&self) -> Result<Vec<PostedOracleBox>>;
}

pub trait LocalDatapointBoxSource {
    fn get_local_oracle_datapoint_box(&self) -> Result<Option<OracleBoxWrapper>>;
}

pub trait VoteBallotBoxesSource {
    fn get_ballot_boxes(&self) -> Result<Vec<VoteBallotBoxWrapper>>;
}

pub trait UpdateBoxSource {
    fn get_update_box(&self) -> Result<UpdateBoxWrapper>;
}

/// Overarching struct which allows for acquiring the state of the whole oracle pool protocol
#[derive(Debug)]
pub struct OraclePool<'a> {
    oracle_datapoint_scan: OracleDatapointScan<'a>,
    local_oracle_datapoint_scan: LocalOracleDatapointScan<'a>,
    local_ballot_box_scan: LocalBallotBoxScan<'a>,
    pool_box_scan: PoolBoxScan<'a>,
    refresh_box_scan: RefreshBoxScan<'a>,
    ballot_boxes_scan: BallotBoxesScan<'a>,
    update_box_scan: UpdateBoxScan<'a>,
}

#[derive(Debug)]
pub struct OracleDatapointScan<'a> {
    scan: OracleTokenScan,
    oracle_box_wrapper_inputs: &'a OracleBoxWrapperInputs,
}

#[derive(Debug)]
pub struct LocalOracleDatapointScan<'a> {
    scan: Scan,
    oracle_box_wrapper_inputs: &'a OracleBoxWrapperInputs,
}

#[derive(Debug)]
pub struct LocalBallotBoxScan<'a> {
    scan: Scan,
    ballot_box_wrapper_inputs: &'a BallotBoxWrapperInputs,
    ballot_token_owner_address: Address,
}

#[derive(Debug)]
pub struct PoolBoxScan<'a> {
    scan: Scan,
    pool_box_wrapper_inputs: &'a PoolBoxWrapperInputs,
}

#[derive(Debug)]
pub struct RefreshBoxScan<'a> {
    scan: Scan,
    refresh_box_wrapper_inputs: &'a RefreshBoxWrapperInputs,
}

#[derive(Debug)]
pub struct BallotBoxesScan<'a> {
    scan: Scan,
    ballot_box_wrapper_inputs: &'a BallotBoxWrapperInputs,
}
#[derive(Debug)]
pub struct UpdateBoxScan<'a> {
    scan: Scan,
    update_box_wrapper_inputs: &'a UpdateBoxWrapperInputs,
}

/// The state of the oracle pool when it is in the Live Epoch stage
#[derive(Debug, Clone)]
pub struct LiveEpochState {
    pub pool_box_epoch_id: EpochCounter,
    pub local_datapoint_box_state: Option<LocalDatapointState>,
    pub latest_pool_datapoint: u64,
    pub latest_pool_box_height: BlockHeight,
}

/// Last posted datapoint box info by the local oracle
#[derive(Debug, Clone)]
pub enum LocalDatapointState {
    Collected {
        height: BlockHeight,
    },
    Posted {
        epoch_id: EpochCounter,
        height: BlockHeight,
    },
}

impl<'a> OraclePool<'a> {
    /// Create a new `OraclePool` struct
    pub fn new() -> std::result::Result<OraclePool<'static>, Error> {
        let pool_config = &POOL_CONFIG;
        let oracle_config = &ORACLE_CONFIG;

        let refresh_box_scan_name = "Refresh Box Scan";

        todo!("get rid of the OraclePool and use individual scans as needed");

        let scan_json = load_scan_ids()?;

        // Create all `Scan` structs for protocol
        let datapoint_scan = OracleTokenScan::load_from_json(&scan_json)?;
        let oracle_datapoint_scan = OracleDatapointScan {
            scan: datapoint_scan,
            oracle_box_wrapper_inputs: &pool_config.oracle_box_wrapper_inputs,
        };
        let local_scan_str = "Local Oracle Datapoint Scan";
        let local_oracle_datapoint_scan = LocalOracleDatapointScan {
            scan: Scan::new(
                "Local Oracle Datapoint Scan",
                &scan_json[local_scan_str].to_string(),
            ),
            oracle_box_wrapper_inputs: &pool_config.oracle_box_wrapper_inputs,
        };

        let local_scan_str = "Local Ballot Box Scan";
        let local_ballot_box_scan = LocalBallotBoxScan {
            scan: Scan::new(local_scan_str, &scan_json[local_scan_str].to_string()),
            ballot_box_wrapper_inputs: &pool_config.ballot_box_wrapper_inputs,
            ballot_token_owner_address: oracle_config.oracle_address.address(),
        };

        let ballot_boxes_scan = BallotBoxesScan {
            scan: Scan::new("Ballot Box Scan", &scan_json["Ballot Box Scan"].to_string()),
            ballot_box_wrapper_inputs: &pool_config.ballot_box_wrapper_inputs,
        };

        let pool_box_scan = PoolBoxScan {
            scan: Scan::new("Pool Box Scan", &scan_json["Pool Box Scan"].to_string()),
            pool_box_wrapper_inputs: &pool_config.pool_box_wrapper_inputs,
        };

        let refresh_box_scan = RefreshBoxScan {
            scan: Scan::new(
                refresh_box_scan_name,
                &scan_json[refresh_box_scan_name].to_string(),
            ),
            refresh_box_wrapper_inputs: &pool_config.refresh_box_wrapper_inputs,
        };

        let update_box_scan = UpdateBoxScan {
            scan: Scan::new("Update Box Scan", &scan_json["Update Box Scan"].to_string()),
            update_box_wrapper_inputs: &pool_config.update_box_wrapper_inputs,
        };

        log::debug!("Scans loaded");

        // Create `OraclePool` struct
        Ok(OraclePool {
            oracle_datapoint_scan,
            local_oracle_datapoint_scan,
            local_ballot_box_scan,
            ballot_boxes_scan,
            pool_box_scan,
            refresh_box_scan,
            update_box_scan,
        })
    }

    /// Get the state of the current oracle pool epoch
    pub fn get_live_epoch_state(&self) -> Result<LiveEpochState> {
        let pool_box = self.get_pool_box_source().get_pool_box()?;
        let epoch_id = pool_box.epoch_counter();

        // Whether datapoint was commit in the current Live Epoch
        let local_datapoint_box_state = self
            .get_local_datapoint_box_source()
            .get_local_oracle_datapoint_box()?
            .map(|local_data_point_box| match local_data_point_box {
                OracleBoxWrapper::Posted(ref posted_box) => LocalDatapointState::Posted {
                    epoch_id: posted_box.epoch_counter(),
                    height: BlockHeight(local_data_point_box.get_box().creation_height),
                },
                OracleBoxWrapper::Collected(_) => LocalDatapointState::Collected {
                    height: BlockHeight(local_data_point_box.get_box().creation_height),
                },
            });

        let latest_pool_datapoint = pool_box.rate() as u64;

        let epoch_state = LiveEpochState {
            pool_box_epoch_id: epoch_id,
            latest_pool_datapoint,
            latest_pool_box_height: BlockHeight(pool_box.get_box().creation_height),
            local_datapoint_box_state,
        };

        Ok(epoch_state)
    }

    pub fn get_pool_box_source(&self) -> &dyn PoolBoxSource {
        &self.pool_box_scan as &dyn PoolBoxSource
    }

    pub fn get_local_ballot_box_source(&self) -> &dyn LocalBallotBoxSource {
        &self.local_ballot_box_scan as &dyn LocalBallotBoxSource
    }

    pub fn get_ballot_boxes_source(&self) -> &dyn VoteBallotBoxesSource {
        &self.ballot_boxes_scan as &dyn VoteBallotBoxesSource
    }

    pub fn get_refresh_box_source(&self) -> &dyn RefreshBoxSource {
        &self.refresh_box_scan as &dyn RefreshBoxSource
    }

    pub fn get_datapoint_boxes_source(&self) -> &dyn DatapointBoxesSource {
        &self.oracle_datapoint_scan as &dyn DatapointBoxesSource
    }

    pub fn get_local_datapoint_box_source(&self) -> &dyn LocalDatapointBoxSource {
        &self.local_oracle_datapoint_scan as &dyn LocalDatapointBoxSource
    }

    pub fn get_update_box_source(&self) -> &dyn UpdateBoxSource {
        &self.update_box_scan as &dyn UpdateBoxSource
    }
}

impl<'a> PoolBoxSource for PoolBoxScan<'a> {
    fn get_pool_box(&self) -> Result<PoolBoxWrapper> {
        let box_wrapper = PoolBoxWrapper::new(
            self.scan
                .get_box()?
                .ok_or(DataSourceError::PoolBoxNotFoundError)?,
            self.pool_box_wrapper_inputs,
        )?;
        Ok(box_wrapper)
    }
}

impl<'a> LocalBallotBoxSource for LocalBallotBoxScan<'a> {
    fn get_ballot_box(&self) -> Result<Option<BallotBoxWrapper>> {
        self.scan
            .get_box()?
            .map(|b| {
                BallotBoxWrapper::new(
                    b,
                    self.ballot_box_wrapper_inputs,
                    &self.ballot_token_owner_address,
                )
                .map_err(Into::into)
            })
            .transpose()
    }
}

impl<'a> RefreshBoxSource for RefreshBoxScan<'a> {
    fn get_refresh_box(&self) -> Result<RefreshBoxWrapper> {
        let box_wrapper = RefreshBoxWrapper::new(
            self.scan
                .get_box()?
                .ok_or(DataSourceError::RefreshBoxNotFoundError)?,
            self.refresh_box_wrapper_inputs,
        )?;
        Ok(box_wrapper)
    }
}

impl<'a> LocalDatapointBoxSource for LocalOracleDatapointScan<'a> {
    fn get_local_oracle_datapoint_box(&self) -> Result<Option<OracleBoxWrapper>> {
        self.scan
            .get_box()?
            .map(|b| OracleBoxWrapper::new(b, self.oracle_box_wrapper_inputs).map_err(Into::into))
            .transpose()
    }
}

impl<'a> VoteBallotBoxesSource for BallotBoxesScan<'a> {
    fn get_ballot_boxes(&self) -> Result<Vec<VoteBallotBoxWrapper>> {
        Ok(self
            .scan
            .get_boxes()?
            .into_iter()
            .map(|ballot_box| {
                Ok(VoteBallotBoxWrapper::new(
                    ballot_box,
                    self.ballot_box_wrapper_inputs,
                )?)
            })
            .filter_map(Result::ok) // Filter out boxes that are not participating in voting
            .collect())
    }
}

impl<'a> UpdateBoxSource for UpdateBoxScan<'a> {
    fn get_update_box(&self) -> Result<UpdateBoxWrapper> {
        let box_wrapper = UpdateBoxWrapper::new(
            self.scan
                .get_box()?
                .ok_or(DataSourceError::UpdateBoxNotFoundError)?,
            self.update_box_wrapper_inputs,
        )?;
        Ok(box_wrapper)
    }
}

impl<'a> DatapointBoxesSource for OracleDatapointScan<'a> {
    fn get_oracle_datapoint_boxes(&self) -> Result<Vec<PostedOracleBox>> {
        let oracle_boxes: Vec<OracleBoxWrapper> = self
            .scan
            .get_boxes()?
            .into_iter()
            .map(|b| OracleBoxWrapper::new(b, self.oracle_box_wrapper_inputs))
            .collect::<std::result::Result<Vec<OracleBoxWrapper>, _>>()?;

        let posted_boxes = oracle_boxes
            .into_iter()
            .filter_map(|b| match b {
                OracleBoxWrapper::Posted(p) => Some(p),
                OracleBoxWrapper::Collected(_) => None,
            })
            .collect();
        Ok(posted_boxes)
    }
}
