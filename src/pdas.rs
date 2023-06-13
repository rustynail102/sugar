use std::ops::Deref;

use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signer::Signer},
    Program,
};
use anyhow::{anyhow, Result};
use mpl_token_metadata::{
    pda::{find_master_edition_account, find_metadata_account},
    state::{Key, MasterEditionV2, Metadata, TokenMetadataAccount, MAX_MASTER_EDITION_LEN},
    utils::try_from_slice_checked,
};

use crate::candy_machine::CANDY_MACHINE_ID;

pub type PdaInfo<T> = (Pubkey, T);

pub struct CollectionPDA {
    pub mint: Pubkey,
    pub candy_machine: Pubkey,
}

pub fn find_metadata_pda(mint: &Pubkey) -> Pubkey {
    let (pda, _bump) = find_metadata_account(mint);

    pda
}

pub fn get_metadata_pda<C: Deref<Target = impl Signer> + Clone>(
    mint: &Pubkey,
    program: &Program<C>,
) -> Result<PdaInfo<Metadata>> {
    let metadata_pubkey = find_metadata_pda(mint);
    let metadata_account = program.rpc().get_account(&metadata_pubkey).map_err(|_| {
        anyhow!(
            "Couldn't find metadata account: {}",
            &metadata_pubkey.to_string()
        )
    })?;
    let metadata = Metadata::safe_deserialize(metadata_account.data.as_slice());
    metadata.map(|m| (metadata_pubkey, m)).map_err(|_| {
        anyhow!(
            "Failed to deserialize metadata account: {}",
            &metadata_pubkey.to_string()
        )
    })
}

pub fn find_master_edition_pda(mint: &Pubkey) -> Pubkey {
    let (pda, _bump) = find_master_edition_account(mint);

    pda
}

pub fn get_master_edition_pda<C: Deref<Target = impl Signer> + Clone>(
    mint: &Pubkey,
    program: &Program<C>,
) -> Result<PdaInfo<MasterEditionV2>> {
    let master_edition_pubkey = find_master_edition_pda(mint);
    let master_edition_account =
        program
            .rpc()
            .get_account(&master_edition_pubkey)
            .map_err(|_| {
                anyhow!(
                    "Couldn't find master edition account: {}",
                    &master_edition_pubkey.to_string()
                )
            })?;
    let master_edition = try_from_slice_checked(
        master_edition_account.data.as_slice(),
        Key::MasterEditionV2,
        MAX_MASTER_EDITION_LEN,
    );
    master_edition
        .map(|m| (master_edition_pubkey, m))
        .map_err(|_| {
            anyhow!(
                "Invalid master edition account: {}",
                &master_edition_pubkey.to_string()
            )
        })
}

pub fn find_candy_machine_creator_pda(candy_machine_id: &Pubkey) -> (Pubkey, u8) {
    // Derive metadata account
    let creator_seeds = &["candy_machine".as_bytes(), candy_machine_id.as_ref()];

    Pubkey::find_program_address(creator_seeds, &CANDY_MACHINE_ID)
}

pub fn find_collection_pda(candy_machine_id: &Pubkey) -> (Pubkey, u8) {
    // Derive collection PDA address
    let collection_seeds = &["collection".as_bytes(), candy_machine_id.as_ref()];

    Pubkey::find_program_address(collection_seeds, &CANDY_MACHINE_ID)
}
