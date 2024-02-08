#![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::sorted_vec_map::SortedVecSet;

/// The state of the petition, which is persisted on-chain.
#[state]
pub struct PeerReviewState {
    owner: Address,
    title: String,
    description: String,
    url_paper: String,
    ratings: SortedVecSet<u8>,
    reviewers: SortedVecSet<Address>,
}


#[init]
pub fn initialize(_ctx: ContractContext, title: String, description: String, url_paper: String) -> PeerReviewState {
    assert_ne!(
        description, "",
        "The description of this peer review cannot be empty."
    );
    assert_ne!(
        title, "",
        "The name of this peer review cannot be empty."
    );
    assert_ne!(
        url_paper, "",
        "The url of the paper of this peer review cannot be empty."
    );
    PeerReviewState {
        owner: _ctx.sender, title,
        description,url_paper,
        ratings: SortedVecSet::new(),
        reviewers: SortedVecSet::new(),
    }
}

#[action(shortname = 0x01)]
pub fn review(ctx: ContractContext, mut state: PeerReviewState, rating: u8) -> PeerReviewState {
    state.reviewers.insert(ctx.sender);
    state.ratings.insert(rating);
    state
}
