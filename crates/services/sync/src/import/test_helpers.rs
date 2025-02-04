#![allow(missing_docs)]

mod counts;
mod pressure_block_importer;
mod pressure_consensus;
mod pressure_peer_to_peer;

use fuel_core_types::{
    blockchain::{
        consensus::{
            Consensus,
            Sealed,
        },
        header::BlockHeader,
        SealedBlockHeader,
    },
    fuel_types::BlockHeight,
};
use rand::{
    rngs::StdRng,
    Rng,
    SeedableRng,
};

pub use counts::{
    Count,
    SharedCounts,
};
use fuel_core_types::services::p2p::PeerId;

pub use pressure_block_importer::PressureBlockImporter;
pub use pressure_consensus::PressureConsensus;
pub use pressure_peer_to_peer::PressurePeerToPeer;

pub fn random_peer() -> PeerId {
    let mut rng = StdRng::seed_from_u64(0xF00DF00D);
    let bytes = rng.gen::<[u8; 32]>().to_vec();
    PeerId::from(bytes)
}

pub fn empty_header<I: Into<BlockHeight>>(i: I) -> SealedBlockHeader {
    let mut header = BlockHeader::default();
    let height = i.into();
    header.consensus.height = height;
    let transaction_tree =
        fuel_core_types::fuel_merkle::binary::in_memory::MerkleTree::new();
    header.application.generated.transactions_root = transaction_tree.root().into();

    let consensus = Consensus::default();
    Sealed {
        entity: header,
        consensus,
    }
}
