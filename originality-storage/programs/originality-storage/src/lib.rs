use anchor_lang::prelude::*;

declare_id!("4CAZ7URST3D1yMU968iZtEEerN4TCZW2eKDvDWqHSZvE");

#[program]
pub mod originality_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.originality_account.image_count = 0;
        Ok(())
    }

    pub fn store_originality(ctx: Context<StoreOriginality>, image_hash: String, originality: bool) -> Result<()> {
        let originality_account = &mut ctx.accounts.originality_account;

        originality_account.add_originality_info(OriginalityInfo {
            image_hash: image_hash.clone(),
            originality,
        })?;

        emit!(OriginalityStored {
            image_hash,
            originality,
        });

        Ok(())
    }

    pub fn get_originality(ctx: Context<GetOriginality>, image_hash: String) -> Result<bool> {
        ctx.accounts.originality_account.get_originality(&image_hash)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + OriginalityAccount::MAX_SIZE)]
    pub originality_account: Account<'info, OriginalityAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StoreOriginality<'info> {
    #[account(mut)]
    pub originality_account: Account<'info, OriginalityAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetOriginality<'info> {
    pub originality_account: Account<'info, OriginalityAccount>,
}

#[account]
pub struct OriginalityAccount {
    pub originality_infos: Vec<OriginalityInfo>,
    pub image_count: u32,
}

impl OriginalityAccount {
    pub const MAX_SIZE: usize = 8 + // discriminator
        4 + // image_count
        4 + // vec length
        1000 * OriginalityInfo::SIZE; // 1000 images max

    pub fn add_originality_info(&mut self, info: OriginalityInfo) -> Result<()> {
        require!(self.image_count < 1000, ErrorCode::TooManyImages);
        if let Some(existing) = self.originality_infos.iter_mut().find(|i| i.image_hash == info.image_hash) {
            *existing = info;
        } else {
            self.originality_infos.push(info);
            self.image_count += 1;
        }
        Ok(())
    }

    pub fn get_originality(&self, image_hash: &str) -> Result<bool> {
        self.originality_infos.iter()
            .find(|info| info.image_hash == image_hash)
            .map(|info| info.originality)
            .ok_or(ErrorCode::ImageNotFound.into())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OriginalityInfo {
    pub image_hash: String,
    pub originality: bool,
}

impl OriginalityInfo {
    pub const SIZE: usize = 32 + // image_hash (assuming max 32 bytes)
        1; // originality (bool)
}

#[event]
pub struct OriginalityStored {
    pub image_hash: String,
    pub originality: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Image not found")]
    ImageNotFound,
    #[msg("Too many images")]
    TooManyImages,
}