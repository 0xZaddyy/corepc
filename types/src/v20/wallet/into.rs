// SPDX-License-Identifier: CC0-1.0
use bitcoin::consensus::encode;
use bitcoin::hashes::hash160;
use bitcoin::hex::FromHex;
use bitcoin::key::PublicKey;
use bitcoin::{
    bip32, Address, BlockHash, ScriptBuf, SignedAmount, Transaction, Txid, WitnessProgram,
    WitnessVersion,
};

use super::{
    AddMultisigAddress, AddMultisigAddressError, GetAddressInfo, GetAddressInfoEmbedded,
    GetAddressInfoEmbeddedError, GetAddressInfoError, GetTransaction, GetTransactionDetail,
    GetTransactionDetailError, GetTransactionError, ListSinceBlock, ListSinceBlockError,
    TransactionItem, TransactionItemError,
};
use crate::model;

impl AddMultisigAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AddMultisigAddress, AddMultisigAddressError> {
        use AddMultisigAddressError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let redeem_script = ScriptBuf::from_hex(&self.redeem_script).map_err(E::RedeemScript)?;

        Ok(model::AddMultisigAddress {
            address,
            redeem_script,
            descriptor: Some(self.descriptor),
            warnings: None, // v23 and later only.
        })
    }
}

impl GetAddressInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfo, GetAddressInfoError> {
        use GetAddressInfoError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program)
        {
            (Some(v), Some(hex)) => {
                if v > u8::MAX as i64 || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version =
                    WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = Vec::from_hex(&hex).map_err(E::WitnessProgramBytes)?;
                let witness_program =
                    WitnessProgram::new(witness_version, &bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            }
            _ => (None, None), // TODO: Think more if catchall is ok.
        };
        let script = self.script.map(|s| s.into_model());
        let redeem_script =
            self.hex.map(|hex| ScriptBuf::from_hex(&hex).map_err(E::Hex)).transpose()?;
        let pubkeys = self
            .pubkeys
            .map(|pubkeys| {
                pubkeys
                    .iter()
                    .map(|s| s.parse::<PublicKey>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(E::Pubkeys)
            })
            .transpose()?;
        let sigs_required =
            self.sigs_required.map(|s| crate::to_u32(s, "sigs_required")).transpose()?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).transpose().map_err(E::Pubkey)?;
        let embedded =
            self.embedded.map(|embedded| embedded.into_model()).transpose().map_err(E::Embedded)?;
        let hd_key_path = self
            .hd_key_path
            .map(|s| s.parse::<bip32::DerivationPath>())
            .transpose()
            .map_err(E::HdKeyPath)?;
        let hd_seed_id =
            self.hd_seed_id.map(|s| s.parse::<hash160::Hash>()).transpose().map_err(E::HdSeedId)?;
        let hd_master_fingerprint = self
            .hd_master_fingerprint
            .map(|s| s.parse::<bip32::Fingerprint>())
            .transpose()
            .map_err(E::HdMasterFingerprint)?;

        Ok(model::GetAddressInfo {
            address,
            script_pubkey,
            is_mine: self.is_mine,
            is_watch_only: self.is_watch_only,
            solvable: Some(self.solvable),
            descriptor: self.descriptor,
            parent_descriptor: None, // v22 and above only.
            is_script: Some(self.is_script),
            is_change: Some(self.is_change),
            is_witness: self.is_witness,
            witness_version,
            witness_program,
            script,
            hex: redeem_script,
            pubkeys,
            sigs_required,
            pubkey,
            embedded,
            is_compressed: self.is_compressed,
            label: None,
            timestamp: self.timestamp,
            hd_key_path,
            hd_seed_id,
            hd_master_fingerprint,
            labels: self.labels,
        })
    }
}

impl GetAddressInfoEmbedded {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfoEmbedded, GetAddressInfoEmbeddedError> {
        use GetAddressInfoEmbeddedError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program)
        {
            (Some(v), Some(hex)) => {
                if v > u8::MAX as i64 || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version =
                    WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = Vec::from_hex(&hex).map_err(E::WitnessProgramBytes)?;
                let witness_program =
                    WitnessProgram::new(witness_version, &bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            }
            _ => (None, None), // TODO: Think more if catchall is ok.
        };
        let script = self.script.map(|s| s.into_model());
        let redeem_script =
            self.hex.map(|hex| ScriptBuf::from_hex(&hex).map_err(E::Hex)).transpose()?;
        let pubkeys = None;
        let sigs_required =
            self.sigs_required.map(|s| crate::to_u32(s, "sigs_required")).transpose()?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).transpose().map_err(E::Pubkey)?;

        Ok(model::GetAddressInfoEmbedded {
            address,
            script_pubkey,
            solvable: self.solvable,
            descriptor: self.descriptor,
            parent_descriptor: None, // v22 and above only.
            is_script: Some(self.is_script),
            is_change: self.is_change,
            is_witness: self.is_witness,
            witness_version,
            witness_program,
            script,
            hex: redeem_script,
            pubkeys,
            sigs_required,
            pubkey,
            is_compressed: self.is_compressed,
            label: None,
            labels: self.labels,
        })
    }
}

impl GetTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransaction, GetTransactionError> {
        use GetTransactionError as E;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let fee = self.fee.map(|fee| SignedAmount::from_btc(fee).map_err(E::Fee)).transpose()?;
        let block_hash =
            self.block_hash.map(|s| s.parse::<BlockHash>().map_err(E::BlockHash)).transpose()?;
        let block_index =
            self.block_index.map(|idx| crate::to_u32(idx, "block_index")).transpose()?;
        let block_height =
            self.block_height.map(|h| crate::to_u32(h, "block_height")).transpose()?;
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wallet_conflicts = self
            .wallet_conflicts
            .into_iter()
            .map(|s| s.parse::<Txid>().map_err(E::WalletConflicts))
            .collect::<Result<Vec<_>, _>>()?;
        let tx = encode::deserialize_hex::<Transaction>(&self.hex).map_err(E::Tx)?;
        let details = self
            .details
            .into_iter()
            .map(|d| d.into_model().map_err(E::Details))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::GetTransaction {
            amount,
            fee, // Option in model
            confirmations: self.confirmations,
            generated: self.generated,
            trusted: self.trusted,
            block_hash,
            block_height,
            block_index,
            block_time: self.block_time,
            txid,
            wtxid: None, // v24 and later only.
            wallet_conflicts,
            replaced_by_txid: None,  // v23 and later only.
            replaces_txid: None,     // v23 and later only.
            mempool_conflicts: None, // v28 and later only.
            to: None,                // v23 and later only.
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable: self.bip125_replaceable.into_model(),
            parent_descriptors: None, // v24 and later only.
            details,
            decoded: self.decoded,
            last_processed_block: None, // v26 and later only.
            tx,
        })
    }
}

impl GetTransactionDetail {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransactionDetail, GetTransactionDetailError> {
        use GetTransactionDetailError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let fee = self.fee.map(|fee| SignedAmount::from_btc(fee).map_err(E::Fee)).transpose()?;

        Ok(model::GetTransactionDetail {
            involves_watch_only: self.involves_watch_only,
            account: self.account,
            address,
            category: self.category.into_model(),
            amount,
            label: self.label,
            vout: self.vout,
            fee,
            abandoned: self.abandoned,
            parent_descriptors: None, // v24 and later only.
        })
    }
}

impl ListSinceBlock {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListSinceBlock, ListSinceBlockError> {
        use ListSinceBlockError as E;

        let transactions = self
            .transactions
            .into_iter()
            .map(|tx| tx.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Transactions)?;
        let removed = self
            .removed
            .into_iter()
            .map(|tx| tx.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Removed)?;
        let last_block = self.last_block.parse::<BlockHash>().map_err(E::LastBlock)?;

        Ok(model::ListSinceBlock { transactions, removed, last_block })
    }
}

impl TransactionItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::TransactionItem, TransactionItemError> {
        use TransactionItemError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let category = self.category.into_model();
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let vout = crate::to_u32(self.vout, "vout")?;
        let fee = self
            .fee
            .map(|f| SignedAmount::from_btc(f).map_err(E::Fee))
            .transpose()? // optional historically
            .unwrap_or_else(|| SignedAmount::from_sat(0));
        let block_hash = self.block_hash.parse::<BlockHash>().map_err(E::BlockHash)?;
        let block_height = crate::to_u32(self.block_height, "block_height")?;
        let block_index = crate::to_u32(self.block_index, "block_index")?;
        let txid = Some(self.txid.parse::<Txid>().map_err(E::Txid)?);
        let wallet_conflicts = self
            .wallet_conflicts
            .into_iter()
            .map(|s| s.parse::<Txid>().map_err(E::WalletConflicts))
            .collect::<Result<Vec<_>, _>>()?;
        let bip125_replaceable = self.bip125_replaceable.into_model();

        Ok(model::TransactionItem {
            involves_watch_only: self.involves_watch_only,
            address: Some(address),
            category,
            amount,
            vout,
            fee,
            confirmations: self.confirmations,
            generated: self.generated,
            trusted: self.trusted,
            block_hash: Some(block_hash),
            block_height: Some(block_height),
            block_index: Some(block_index),
            block_time: Some(self.block_time),
            txid,
            wtxid: None,
            wallet_conflicts: Some(wallet_conflicts),
            replaced_by_txid: None,
            replaces_txid: None,
            mempool_conflicts: None,
            to: self.to,
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable,
            parent_descriptors: None,
            abandoned: self.abandoned,
            label: self.label,
        })
    }
}
