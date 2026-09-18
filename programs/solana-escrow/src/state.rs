use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Trade {
    pub approved_by_party_1: bool,
    pub approved_by_party_2: bool,
}
