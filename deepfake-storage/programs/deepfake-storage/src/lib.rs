use anchor_lang::prelude::*;

declare_id!("Aqyqt3mnUVMDErUPvQm9e4LDWHHtJKpLXsBhkumbk6L2");

#[program]
pub mod deepfake_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.deepfake_account.image_count = 0;
        Ok(())
    }

    pub fn store_image(ctx: Context<StoreImage>, image_hash: String, deepfake_value: u8) -> Result<()> {
        require!(deepfake_value == 1 || deepfake_value == 2 || deepfake_value == 3, ErrorCode::InvalidDeepfakeValue);

        let deepfake_account = &mut ctx.accounts.deepfake_account;
        let timestamp = Clock::get()?.unix_timestamp as u64;

        deepfake_account.add_image_info(ImageInfo {
            image_hash: image_hash.clone(),
            deepfake_value,
            timestamp,
        })?;

        emit!(ImageAdded {
            image_hash,
            deepfake_value,
            timestamp,
        });

        Ok(())
    }

    pub fn get_deepfake_value(ctx: Context<GetDeepfakeValue>, image_hash: String) -> Result<u8> {
        ctx.accounts.deepfake_account.get_deepfake_value(&image_hash)
    }

    pub fn get_image_timestamp(ctx: Context<GetImageTimestamp>, image_hash: String) -> Result<u64> {
        ctx.accounts.deepfake_account.get_image_timestamp(&image_hash)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + DeepfakeAccount::MAX_SIZE)]
    pub deepfake_account: Account<'info, DeepfakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StoreImage<'info> {
    #[account(mut)]
    pub deepfake_account: Account<'info, DeepfakeAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetDeepfakeValue<'info> {
    pub deepfake_account: Account<'info, DeepfakeAccount>,
}

#[derive(Accounts)]
pub struct GetImageTimestamp<'info> {
    pub deepfake_account: Account<'info, DeepfakeAccount>,
}

#[account]
pub struct DeepfakeAccount {
    pub image_infos: Vec<ImageInfo>,
    pub image_count: u32,
}

impl DeepfakeAccount {
    pub const MAX_SIZE: usize = 8 + // discriminator
        4 + // image_count
        4 + // vec length
        1000 * ImageInfo::SIZE; // 1000 images max

    pub fn add_image_info(&mut self, info: ImageInfo) -> Result<()> {
        require!(self.image_count < 1000, ErrorCode::TooManyImages);
        if let Some(existing) = self.image_infos.iter_mut().find(|i| i.image_hash == info.image_hash) {
            *existing = info;
        } else {
            self.image_infos.push(info);
            self.image_count += 1;
        }
        Ok(())
    }

    pub fn get_deepfake_value(&self, image_hash: &str) -> Result<u8> {
        self.image_infos.iter()
            .find(|info| info.image_hash == image_hash)
            .map(|info| info.deepfake_value)
            .ok_or(ErrorCode::ImageNotFound.into())
    }

    pub fn get_image_timestamp(&self, image_hash: &str) -> Result<u64> {
        self.image_infos.iter()
            .find(|info| info.image_hash == image_hash)
            .map(|info| info.timestamp)
            .ok_or(ErrorCode::ImageNotFound.into())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ImageInfo {
    pub image_hash: String,
    pub deepfake_value: u8,
    pub timestamp: u64,
}

impl ImageInfo {
    pub const SIZE: usize = 32 + // image_hash (assuming max 32 bytes)
        1 + // deepfake_value
        8; // timestamp
}

#[event]
pub struct ImageAdded {
    pub image_hash: String,
    pub deepfake_value: u8,
    pub timestamp: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid deepfake value")]
    InvalidDeepfakeValue,
    #[msg("Image not found")]
    ImageNotFound,
    #[msg("Too many images")]
    TooManyImages,
}