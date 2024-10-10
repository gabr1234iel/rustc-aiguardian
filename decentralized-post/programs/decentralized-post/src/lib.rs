use anchor_lang::prelude::*;

declare_id!("AfCDBjnYCyHh7Hb9YiKx8NVQXA7dFfaaY5yFFF8DabJb");

#[program]
pub mod decentralized_post {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let post_account = &mut ctx.accounts.post_account;
        post_account.next_post_id = 1;
        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, ipfs_hash: String, image_hash: String, content: String, world_id: String) -> Result<()> {
        let post_account = &mut ctx.accounts.post_account;
        let user = &ctx.accounts.user;
    
        let post_id = post_account.next_post_id;
        let user_address = user.key().to_string();
        let timestamp = Clock::get()?.unix_timestamp as u64;
    
        let post = Post {
            post_id,
            user_address: user_address.clone(),
            ipfs_hash: ipfs_hash.clone(),
            image_hash: image_hash.clone(),
            content: content.clone(),
            timestamp,
            world_id: world_id.clone(),
        };
    
        post_account.add_post(post)?;
    
        emit!(PostCreated {
            post_id,
            user_address,
            ipfs_hash,
            image_hash,
            content,
            timestamp,
            world_id,
        });
    
        post_account.next_post_id += 1;
        Ok(())
    }

    pub fn get_post(ctx: Context<GetPost>, post_id: u64) -> Result<Post> {
        let post_account = &ctx.accounts.post_account;
        post_account.get_post(post_id)
    }

    pub fn get_posts_descending(ctx: Context<GetPostsDescending>, limit: u8) -> Result<Vec<Post>> {
        let post_account = &ctx.accounts.post_account;
        post_account.get_posts_descending(limit)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + PostAccount::MAX_SIZE)]
    pub post_account: Account<'info, PostAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(mut)]
    pub post_account: Account<'info, PostAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetPost<'info> {
    pub post_account: Account<'info, PostAccount>,
}

#[derive(Accounts)]
pub struct GetPostsDescending<'info> {
    pub post_account: Account<'info, PostAccount>,
}

#[account]
pub struct PostAccount {
    pub posts: Vec<Post>,
    pub next_post_id: u64,
}

impl PostAccount {
    pub const MAX_SIZE: usize = 8 + // discriminator
        8 + // next_post_id
        4 + // vec length
        100 * Post::SIZE; // 100 posts max

    pub fn add_post(&mut self, post: Post) -> Result<()> {
        require!(self.posts.len() < 100, ErrorCode::TooManyPosts);
        self.posts.push(post);
        Ok(())
    }

    pub fn get_post(&self, post_id: u64) -> Result<Post> {
        self.posts.iter()
            .find(|&post| post.post_id == post_id)
            .cloned()
            .ok_or(ErrorCode::PostNotFound.into())
    }

    pub fn get_posts_descending(&self, limit: u8) -> Result<Vec<Post>> {
        Ok(self.posts.iter()
            .rev()
            .take(limit as usize)
            .cloned()
            .collect())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Post {
    pub post_id: u64,
    pub user_address: String,
    pub ipfs_hash: String,
    pub image_hash: String,
    pub content: String,
    pub timestamp: u64,
    pub world_id: String,
}

impl Post {
    pub const SIZE: usize = 8 + // post_id
        32 + // user_address (Pubkey)
        32 + // ipfs_hash (assuming max 32 bytes)
        32 + // image_hash (assuming max 32 bytes)
        200 + // content (assuming max 200 bytes)
        8 + // timestamp
        32; // world_id (assuming max 32 bytes)
}

#[event]
pub struct PostCreated {
    pub post_id: u64,
    pub user_address: String,
    pub ipfs_hash: String,
    pub image_hash: String,
    pub content: String,
    pub timestamp: u64,
    pub world_id: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Post not found")]
    PostNotFound,
    #[msg("Too many posts")]
    TooManyPosts,
}