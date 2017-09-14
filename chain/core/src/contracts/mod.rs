// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! System contracts.

pub mod node_manager;
pub mod account_manager;
pub mod quota_manager;

pub use self::account_manager::AccountManager;
pub use self::node_manager::NodeManager;
pub use self::quota_manager::QuotaManager;
use rustc_serialize::hex::ToHex;

use util::{Address, U256, H160};

/// Parse solidity return data `String` to rust `Vec<Address>`
pub fn parse_string_to_addresses(data: &Vec<u8>) -> Vec<Address> {
    let mut nodes = Vec::new();
    trace!("data.len is {:?}", data.len());
    if data.len() > 0 {
        let len_len = U256::from(&data[0..32]).as_u64() as usize;
        trace!("len_len is {:?}", len_len);
        if len_len <= 32 {
            let len = U256::from(&data[32..32 + len_len]).as_u64() as usize;
            let num = len / 20;
            for i in 0..num {
                let node = H160::from(&data[32 + len_len + i * 20..32 + len_len + (i + 1) * 20]);
                trace!("node {:?}", node);
                if node != H160::default() {
                    nodes.push(node);
                }
            }
        }
    }
    nodes
}

/// parse quota
pub fn parse_string_to_quota(data: &Vec<u8>) -> Vec<String> {
    let mut quotas = Vec::new();
    trace!("parse_string_to_quota data.len is {:?}", data.len());
    if data.len() > 0 {
        let len_len = U256::from(&data[0..32]).as_u64() as usize;
        trace!("parse_string_to_quota len_len is {:?}", len_len);
        if len_len <= 32 {
            let len = U256::from(&data[32..32 + len_len]).as_u64() as usize;
            let num = len / 4;
            for i in 0..num {
                let quota = ToHex::to_hex(&data[32 + len_len + i * 4..32 + len_len + (i + 1) * 4]);
                trace!("parse_string_to_addresses quota {:?}", quota);
                if !quota.is_empty() {
                    quotas.push(quota);
                }
            }
        }
    }
    quotas
}
