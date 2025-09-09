// SPDX-License-Identifier: CC0-1.0

use bitcoin::{CompactTarget, Target, Weight};

use super::{GetMiningInfo, GetMiningInfoError, NextBlockInfo, NextBlockInfoError};
use crate::model;

impl GetMiningInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMiningInfo, GetMiningInfoError> {
        use GetMiningInfoError as E;

        let current_block_weight = self.current_block_weight.map(Weight::from_wu);
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let target = Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?;

        let next = self.next.into_model().map_err(E::Next)?;

        Ok(model::GetMiningInfo {
            blocks: self.blocks,
            current_block_weight,
            current_block_tx: self.current_block_tx,
            bits: Some(bits),
            difficulty: self.difficulty,
            target: Some(target),
            network_hash_ps: self.network_hash_ps,
            pooled_tx: self.pooled_tx,
            chain: self.chain,
            signet_challenge: self.signet_challenge,
            next: Some(next),
            warnings: self.warnings,
        })
    }
}

impl NextBlockInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::NextBlockInfo, NextBlockInfoError> {
        use NextBlockInfoError as E;

        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let target = Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?;

        Ok(model::NextBlockInfo { height: self.height, bits, difficulty: self.difficulty, target })
    }
}
