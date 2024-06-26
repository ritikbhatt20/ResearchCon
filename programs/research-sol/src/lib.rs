use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::{
    solana_program::program_error::ProgramError,
};

mod error;

use crate::{error::*};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("FdE5uyJNjYc1isahWvmcdWhAQmN9GuyMA936Po3B9Aho");

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

    pub fn signup_user(ctx: Context<SignupUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        if name.is_empty() || avatar.is_empty() {
            return Err(ProgramError::Custom(ResearchError::InvalidInput as u32));
        }

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String, timestamp: u64, links: Option<Vec<String>>) -> ProgramResult {
        let blog_account = &mut ctx.accounts.blog_account;
        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        if title.is_empty() || content.is_empty() {
            return Err(ProgramError::Custom(ResearchError::InvalidInput as u32));       
        }

        post_account.title = title;
        post_account.content = content;
        post_account.user = user_account.key();
        post_account.authority = authority.key();
        post_account.pre_post_key = blog_account.current_post_key;
        post_account.timestamp = timestamp;
        post_account.links = links; // Set the list of links

        blog_account.current_post_key = post_account.key();

        user_account.num_posts += 1;

        emit!(PostEvent {
            label: "CREATE".to_string(),
            post_id: post_account.key(),
            next_post_id: None
        });

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

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = authority, space = 8 + 50 + 500 + 32 + 32 + 32 + 32 + 32 + 32)]
    pub post_account: Account<'info, PostState>,
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, UserState>,
    #[account(mut)]
    pub blog_account: Account<'info, BlogState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct SignupUser<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 120  + 32)]
    pub user_account: Account<'info, UserState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct PostEvent {
    pub label: String,
    pub post_id: Pubkey,
    pub next_post_id: Option<Pubkey>,
}

#[account]
pub struct BlogState {
    pub current_post_key: Pubkey,
    pub authority: Pubkey,
    pub initialized: bool,
}

#[account]
pub struct UserState {
    pub name: String,
    pub avatar: String,
    pub authority: Pubkey,
    pub num_posts: u64, 
}

#[account]
pub struct PostState {
    pub title: String,
    pub content: String,
    pub user: Pubkey,
    pub pre_post_key: Pubkey,
    pub authority: Pubkey,
    pub timestamp: u64,
    pub links: Option<Vec<String>>, 
}