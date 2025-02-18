// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Constants for the Ethereum protocol.
extern crate alloc;
use alloc::{collections::BTreeMap, str::FromStr};

use alloy_primitives::Address;
use anyhow::{bail, Result};
use once_cell::unsync::Lazy;
use raiko_primitives::{uint, BlockNumber, ChainId, U256};
use revm::primitives::SpecId;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use crate::no_std::*;

/// U256 representation of 0.
pub const ZERO: U256 = U256::ZERO;
/// U256 representation of 1.
pub const ONE: U256 = uint!(1_U256);

/// Maximum size of extra data.
pub const MAX_EXTRA_DATA_BYTES: usize = 32;

/// Maximum allowed block number difference for the `block_hash` call.
pub const MAX_BLOCK_HASH_AGE: u64 = 256;

/// Multiplier for converting gwei to wei.
pub const GWEI_TO_WEI: U256 = uint!(1_000_000_000_U256);

/// The Ethereum mainnet specification.
pub const ETH_MAINNET_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| {
    ChainSpec {
        chain_id: 1,
        max_spec_id: SpecId::CANCUN,
        hard_forks: BTreeMap::from([
            (SpecId::FRONTIER, ForkCondition::Block(0)),
            // previous versions not supported
            (SpecId::MERGE, ForkCondition::Block(15537394)),
            (SpecId::SHANGHAI, ForkCondition::Block(17034870)),
            (SpecId::CANCUN, ForkCondition::Timestamp(1710338135)),
        ]),
        eip_1559_constants: Eip1559Constants {
            base_fee_change_denominator: uint!(8_U256),
            base_fee_max_increase_denominator: uint!(8_U256),
            base_fee_max_decrease_denominator: uint!(8_U256),
            elasticity_multiplier: uint!(2_U256),
        },
        l1_contract: None,
        l2_contract: None,
        sgx_verifier_address: None,
        genesis_time: 0u64,
        seconds_per_slot: 1u64,
    }
});

/// The Taiko A6 specification.
pub const TAIKO_A6_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| ChainSpec {
    chain_id: 167008,
    max_spec_id: SpecId::SHANGHAI,
    hard_forks: BTreeMap::from([
        (SpecId::SHANGHAI, ForkCondition::Block(0)),
        (SpecId::CANCUN, ForkCondition::TBD),
    ]),
    eip_1559_constants: Eip1559Constants {
        base_fee_change_denominator: uint!(8_U256),
        base_fee_max_increase_denominator: uint!(8_U256),
        base_fee_max_decrease_denominator: uint!(8_U256),
        elasticity_multiplier: uint!(2_U256),
    },
    l1_contract: Some(Address::from_str("0xB20BB9105e007Bd3E0F73d63D4D3dA2c8f736b77").unwrap()),
    l2_contract: Some(Address::from_str("0x1670080000000000000000000000000000010001").unwrap()),
    sgx_verifier_address: Some(
        Address::from_str("0x558E38a3286916934Cb63ced04558A52F7Ce67a9").unwrap(),
    ),
    genesis_time: 0u64,
    seconds_per_slot: 1u64,
});

/// The Taiko A7 specification.
pub const TAIKO_A7_CHAIN_SPEC: Lazy<ChainSpec> = Lazy::new(|| ChainSpec {
    chain_id: 167009,
    max_spec_id: SpecId::SHANGHAI,
    hard_forks: BTreeMap::from([
        (SpecId::SHANGHAI, ForkCondition::Block(0)),
        (SpecId::CANCUN, ForkCondition::TBD),
    ]),
    eip_1559_constants: Eip1559Constants {
        base_fee_change_denominator: uint!(8_U256),
        base_fee_max_increase_denominator: uint!(8_U256),
        base_fee_max_decrease_denominator: uint!(8_U256),
        elasticity_multiplier: uint!(2_U256),
    },
    l1_contract: Some(Address::from_str("0x79C9109b764609df928d16fC4a91e9081F7e87DB").unwrap()),
    l2_contract: Some(Address::from_str("0x1670090000000000000000000000000000010001").unwrap()),
    sgx_verifier_address: Some(
        Address::from_str("0x532EFBf6D62720D0B2a2Bb9d11066E8588cAE6D9").unwrap(),
    ),
    genesis_time: 1695902400u64,
    seconds_per_slot: 12u64,
});

pub fn get_network_spec(network: Network) -> ChainSpec {
    match network {
        Network::Ethereum => ETH_MAINNET_CHAIN_SPEC.clone(),
        Network::TaikoA6 => TAIKO_A6_CHAIN_SPEC.clone(),
        Network::TaikoA7 => TAIKO_A7_CHAIN_SPEC.clone(),
    }
}

/// The condition at which a fork is activated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForkCondition {
    /// The fork is activated with a certain block.
    Block(BlockNumber),
    /// The fork is activated with a specific timestamp.
    Timestamp(u64),
    /// The fork is not yet active.
    TBD,
}

impl ForkCondition {
    /// Returns whether the condition has been met.
    pub fn active(&self, block_no: BlockNumber, timestamp: u64) -> bool {
        match self {
            ForkCondition::Block(block) => *block <= block_no,
            ForkCondition::Timestamp(ts) => *ts <= timestamp,
            ForkCondition::TBD => false,
        }
    }
}

/// [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559) parameters.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Eip1559Constants {
    pub base_fee_change_denominator: U256,
    pub base_fee_max_increase_denominator: U256,
    pub base_fee_max_decrease_denominator: U256,
    pub elasticity_multiplier: U256,
}

impl Default for Eip1559Constants {
    /// Defaults to Ethereum network values
    fn default() -> Self {
        Self {
            base_fee_change_denominator: uint!(8_U256),
            base_fee_max_increase_denominator: uint!(8_U256),
            base_fee_max_decrease_denominator: uint!(8_U256),
            elasticity_multiplier: uint!(2_U256),
        }
    }
}

/// Specification of a specific chain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChainSpec {
    pub chain_id: ChainId,
    pub max_spec_id: SpecId,
    pub hard_forks: BTreeMap<SpecId, ForkCondition>,
    pub eip_1559_constants: Eip1559Constants,
    pub l1_contract: Option<Address>,
    pub l2_contract: Option<Address>,
    pub sgx_verifier_address: Option<Address>,
    pub genesis_time: u64,
    pub seconds_per_slot: u64,
}

impl ChainSpec {
    /// Creates a new configuration consisting of only one specification ID.
    pub fn new_single(
        chain_id: ChainId,
        spec_id: SpecId,
        eip_1559_constants: Eip1559Constants,
    ) -> Self {
        ChainSpec {
            chain_id,
            max_spec_id: spec_id,
            hard_forks: BTreeMap::from([(spec_id, ForkCondition::Block(0))]),
            eip_1559_constants,
            l1_contract: None,
            l2_contract: None,
            sgx_verifier_address: None,
            genesis_time: 0u64,
            seconds_per_slot: 1u64,
        }
    }
    /// Returns the network chain ID.
    pub fn chain_id(&self) -> ChainId {
        self.chain_id
    }
    /// Returns the [SpecId] for a given block number and timestamp or an error if not
    /// supported.
    pub fn active_fork(&self, block_no: BlockNumber, timestamp: u64) -> Result<SpecId> {
        match self.spec_id(block_no, timestamp) {
            Some(spec_id) => {
                if spec_id > self.max_spec_id {
                    bail!("expected <= {:?}, got {:?}", self.max_spec_id, spec_id);
                } else {
                    Ok(spec_id)
                }
            }
            None => bail!("no supported fork for block {}", block_no),
        }
    }
    /// Returns the Eip1559 constants
    pub fn gas_constants(&self) -> &Eip1559Constants {
        &self.eip_1559_constants
    }

    fn spec_id(&self, block_no: BlockNumber, timestamp: u64) -> Option<SpecId> {
        for (spec_id, fork) in self.hard_forks.iter().rev() {
            if fork.active(block_no, timestamp) {
                return Some(*spec_id);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Network {
    /// The Ethereum Mainnet
    #[default]
    Ethereum,
    /// Taiko A6 tesnet
    TaikoA6,
    /// Taiko A7 tesnet
    TaikoA7,
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" => Ok(Network::Ethereum),
            "taiko_a6" => Ok(Network::TaikoA6),
            "taiko_a7" => Ok(Network::TaikoA7),
            #[allow(clippy::needless_return)]
            _ => bail!("Unknown network"),
        }
    }
}

impl ToString for Network {
    fn to_string(&self) -> String {
        match self {
            Network::Ethereum => String::from("ethereum"),
            Network::TaikoA6 => String::from("taiko_a6"),
            Network::TaikoA7 => String::from("taiko_a7"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revm_spec_id() {
        assert!(ETH_MAINNET_CHAIN_SPEC.spec_id(15537393, 0) < Some(SpecId::MERGE));
        assert_eq!(
            ETH_MAINNET_CHAIN_SPEC.spec_id(15537394, 0),
            Some(SpecId::MERGE)
        );
        assert_eq!(
            ETH_MAINNET_CHAIN_SPEC.spec_id(17034869, 0),
            Some(SpecId::MERGE)
        );
        assert_eq!(
            ETH_MAINNET_CHAIN_SPEC.spec_id(17034870, 0),
            Some(SpecId::SHANGHAI)
        );
    }
}
