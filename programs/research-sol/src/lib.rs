use anchor_lang::prelude::*;

declare_id!("7AX5ZbemiAUG4WVdh12stGpsS6E4W8WeCbcDpLiyoWE4");

#[program]
mod blog_sol {

    use super::*;

    pub fn init_blog(ctx: Context<InitBlog>) -> ProgramResult {
        let blog_account = &mut ctx.accounts.blog_account;
        let genesis_post_account = &mut ctx.accounts.genesis_post_account;
        let authority = &mut ctx.accounts.authority;

        if blog_account.initialized {
            // return err!(ResearchError::BlogAlreadyInitialized);
            return Err(ProgramError::Custom(ResearchError::BlogAlreadyInitialized as u32));
        }

        blog_account.authority = authority.key();
        blog_account.current_post_key = genesis_post_account.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBlog<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32)]
    pub blog_account: Account<'info, BlogState>,
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32 + 8)]
    pub genesis_post_account: Account<'info, PostState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BlogState {
    pub current_post_key: Pubkey,
    pub authority: Pubkey,
    pub initialized: bool,
}