use anchor_lang::prelude::*;

use spl_account_compression::{
    cpi::{accounts::Initialize, init_empty_merkle_tree},
    program::SplAccountCompression,
    Noop,
};

declare_id!("7fh5eBGRAztZJ8UB3dQuxxZtCBu7pNRUkno5wsagA7Hu");

#[program]
pub mod spl_account_compression_idl_issue {
    use super::*;

    pub fn initialize_merkle_tree(
        ctx: Context<InitializeMerkleTree>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        let cpi_program: AccountInfo<'_> =
            ctx.accounts.account_compression_program.to_account_info();

        let cpi_accounts: Initialize<'_> = Initialize {
            merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
            authority: ctx.accounts.tree_authority.to_account_info(),
            noop: ctx.accounts.noop_program.to_account_info(),
        };

        let merkle_tree: Pubkey = ctx.accounts.merkle_tree.key();

        let signer_seeds: &[&[&[u8]]] = &[&[merkle_tree.as_ref(), &[ctx.bumps.tree_authority]]];

        let cpi_ctx: CpiContext<'_, '_, '_, '_, Initialize<'_>> =
            CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        init_empty_merkle_tree(cpi_ctx, max_depth, max_buffer_size)?;

        msg!("Merkle tree initialized successfully!");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMerkleTree<'info> {
    #[account(mut)]
    pub merkle_tree: Signer<'info>,

    /// CHECK: This is the authority that will control the tree, set to the program itself
    #[account(
        seeds = [b"tree_authority"],
        bump
    )]
    pub tree_authority: AccountInfo<'info>,

    pub noop_program: Program<'info, Noop>,

    pub account_compression_program: Program<'info, SplAccountCompression>,

    pub system_program: Program<'info, System>,
}
