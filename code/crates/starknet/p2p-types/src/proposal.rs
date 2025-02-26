use bytes::Bytes;
use malachitebft_core_types::Round;
use malachitebft_proto as proto;
use malachitebft_starknet_p2p_proto as p2p_proto;

use crate::{Address, BlockHash, Height};

/// A proposal for a value in a round
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub height: Height,
    pub round: Round,
    pub block_hash: BlockHash,
    pub pol_round: Round,
    pub proposer: Address,
}

impl Proposal {
    pub fn new(
        height: Height,
        round: Round,
        block_hash: BlockHash,
        pol_round: Round,
        proposer: Address,
    ) -> Self {
        Self {
            height,
            round,
            block_hash,
            pol_round,
            proposer,
        }
    }

    pub fn to_sign_bytes(&self) -> Bytes {
        proto::Protobuf::to_bytes(self).unwrap()
    }
}

impl proto::Protobuf for Proposal {
    type Proto = p2p_proto::Proposal;

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn to_proto(&self) -> Result<Self::Proto, proto::Error> {
        Ok(Self::Proto {
            block_number: self.height.block_number,
            fork_id: self.height.fork_id,
            round: self.round.as_u32().expect("round should not be nil"),
            block_hash: Some(self.block_hash.to_proto()?),
            pol_round: self.pol_round.as_u32(),
            proposer: Some(self.proposer.to_proto()?),
        })
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn from_proto(proto: Self::Proto) -> Result<Self, proto::Error> {
        Ok(Self {
            height: Height::new(proto.block_number, proto.fork_id),
            round: Round::new(proto.round),
            block_hash: BlockHash::from_proto(
                proto
                    .block_hash
                    .ok_or_else(|| proto::Error::missing_field::<Self::Proto>("block_hash"))?,
            )?,
            pol_round: Round::from(proto.pol_round),
            proposer: Address::from_proto(
                proto
                    .proposer
                    .ok_or_else(|| proto::Error::missing_field::<Self::Proto>("proposer"))?,
            )?,
        })
    }
}
