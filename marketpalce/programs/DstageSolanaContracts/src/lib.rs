use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_lang::error_code;
use anchor_spl::token::{MintTo, Token, Transfer, Burn};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("91RwocfXVQjBqryiad78U1hoMJ9Zc7WzjgQAnyWMboae");

// Warnings
// Check Burn Warning 


// TO DO
// Add Signers for Each Struct/Instruction
// Print Editions from Master Edition
// Fixed Price Sale (Transfer Sol From Wallet to Wallet)
// Add Bids and Store Balance


#[program]
pub mod dstage_solana_contracts 
{
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        msg!("Initializing Mint Ticket");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
    
        msg!("Account Info Assigned");
        
        // Could be Multiple Creators
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 50,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 50,
            },
        ];
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("SNDY");
        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                title,
                symbol,
                uri,
                Some(creator),
                // Royalty Fee of NFT
                350,
                true,
                false,
                None,
                None,
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ];
            msg!("Master Edition Account Infos Assigned");
            invoke(
                &create_master_edition_v3(
                    ctx.accounts.token_metadata_program.key(),
                    ctx.accounts.master_edition.key(),
                    ctx.accounts.mint.key(),
                    ctx.accounts.payer.key(),
                    ctx.accounts.mint_authority.key(),
                    ctx.accounts.metadata.key(),
                    ctx.accounts.payer.key(),
                    Some(0),
                ),
                master_edition_infos.as_slice(),
            )?;
            msg!("Master Edition Nft Minted !!!");
            
            Ok(())
        }


        pub fn transfer_nft(ctx: Context<TransferToken>) -> Result<()> {
            let transfer_instruction = Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.from_authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);
            anchor_spl::token::transfer(cpi_ctx, 1)?;
            msg!("Transferd NFT Token");
            Ok(())
        }

        pub fn burn_nft (ctx: Context<BurnNFT>) -> Result<()> {
            let burn_instruction = Burn {
                mint: ctx.accounts.mint_address.to_account_info(),
                from: ctx.accounts.from.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, burn_instruction);
            anchor_spl::token::burn(cpi_ctx, 1)?;
            msg!("NFT {:?} Burnt successfully", ctx.accounts.mint_address);
            Ok(())
        }


        pub fn place_nft_for_fixed_price(ctx: Context<PlaceNFTForFixedPrice>, nft_price: u64) -> Result<()> {
            msg!("Here");
            // Make sure NFT exist and should be not on sale
            let nft_info = &mut ctx.accounts.nft_info;
            nft_info.nft_price = nft_price;
            nft_info.sale_state = nft_info.fixed_price_state();
            nft_info.nft_authority = ctx.accounts.authority.key();
            msg!("NFT {:?} Placed For Fixed Price ", ctx.accounts.mint_key);
            Ok(())
        }

        pub fn prchase_nft_against_fixed_price(ctx: Context<PurchaseNFTAgainstFixedPrice>, nft_price: u64 ) -> Result<()> {
            let nft_info = &mut ctx.accounts.nft_info;
            let nft_price = nft_info.nft_price; 
            let from_ata = &mut ctx.accounts.from_ata;
            let to_ata = &mut ctx.accounts.to_ata;
            let nft_authority = &mut ctx.accounts.nft_authority;
            let price_payer = &mut ctx.accounts.price_payer;

            // **price_payer.to_account_info().try_borrow_mut_lamports()? = price_payer.to_account_info().lamports().checked_sub(nft_price).ok_or(ErrorCode::InsufficientBalance)?;
            // **nft_authority.to_account_info().try_borrow_mut_lamports()? = nft_authority.to_account_info().lamports().checked_sub(nft_price).ok_or(ErrorCode::InsufficientBalance)?;

            // Transfer nft to purchaser address
            let transfer_ix = Transfer {
                from: from_ata.to_account_info(),
                to: to_ata.to_account_info(),
                authority: nft_authority.to_account_info(),
            };
            let program_id = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(program_id, transfer_ix);
            anchor_spl::token::transfer(cpi_ctx , 1)?;
            msg!("NFT Purchased Successfully");
            Ok(())
        }

        pub fn remove_nft_from_sale(ctx: Context<RemoveNftFromSale>) -> Result<()> {
            let nft_info = &mut ctx.accounts.nft_info;
            nft_info.sale_state = nft_info.remove_from_sale();
            let nft_id = &ctx.accounts.mint_key;
            msg!("NFT {:?} removed from sale", nft_id);
            Ok(())
        } 

        pub fn place_nft_for_timed_auction(ctx: Context<PlaceNftForTimedAuction> , auction_start_time: i64, auction_end_time: i64, minimum_bet_amount: u64)-> Result<()> {

            let current_time = Clock::get().unwrap().unix_timestamp;
            
            require!(auction_end_time > current_time , InvalidTime);
            require!(auction_start_time >= current_time, InvalidTime);
            // let authority = ctx.accounts.authority;
            let nft_info = &mut ctx.accounts.nft_info;
            let nft_state = nft_info.remove_from_sale();
            require!(nft_info.nft_authority == ctx.accounts.authority.key(), OnlyOwnerCan);
            
            require!(nft_info.sale_state == nft_state , NFTAlreadyOnSale);
            nft_info.min_bid_amount = minimum_bet_amount;
            nft_info.sale_state = nft_info.auction_state();
            nft_info.auction_start_time = auction_start_time;
            nft_info.auction_end_time = auction_end_time;
            msg!("NFT Placed for Timed Auction");
            Ok(())

        }

        pub fn add_bids(ctx: Context<AddBid>, bid_amount: u64 ) -> Result<()> {
            
            let nft_info = &mut ctx.accounts.nft_info;
            let auction_state = nft_info.auction_state();
            require!(nft_info.sale_state == auction_state , NFTNotOnSale);
            require!(bid_amount >= nft_info.min_bid_amount, LessBidAmount);
            let current_time = Clock::get().unwrap().unix_timestamp;
            require!(current_time >= nft_info.auction_start_time && current_time <= nft_info.auction_end_time, InvalidTime);
            Ok(())
        }
        
}



#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK:  
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK:  
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK:  
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK:  
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK:  
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK:  
    pub rent: AccountInfo<'info>,
    /// CHECK:  
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}


#[derive(Accounts)]
pub struct TransferToken<'info> {
    /// CHECK: 
    #[account(mut)]
    pub from : UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    /// CHECK:
    pub from_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnNFT<'info> {
    #[account(mut)]
    /// CHECK:
    pub mint_address: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub from: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,   
}


#[account]
pub struct NftInfo {
    auction_start_time: i64, 
    auction_end_time: i64,
    nft_price: u64, 
    sale_state: SaleState, 
    nft_authority: Pubkey,
    min_bid_amount: u64,
    
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum SaleState {
    FixedPriceSale, 
    TimedAuctionSale, 
    NotOnSale, 
}

impl NftInfo {
    fn fixed_price_state(&mut self) -> SaleState 
    {
        SaleState::FixedPriceSale
    }
    fn auction_state(&mut self) -> SaleState {
        SaleState::TimedAuctionSale        
    }
    fn remove_from_sale(&mut self) -> SaleState {
        SaleState::NotOnSale
    }
    fn return_authority(&mut self) -> Pubkey {
        self.nft_authority
    }

} 
impl Default for SaleState {
    fn default() -> Self {
        Self::NotOnSale
    }
} 



#[derive(Accounts)]
// Check the author is placing for fixed price
pub struct PlaceNFTForFixedPrice <'info> {
    
    #[account(init, payer = authority, 
        space = 64 + 64 + 64 + 64,
        seeds = [b"config", mint_key.key().as_ref()] , bump
    )]
    pub nft_info: Account<'info , NftInfo>,
    /// CHECK:    
    pub mint_key: AccountInfo<'info>,
    /// CHECK:
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>, 
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct PurchaseNFTAgainstFixedPrice<'info> {
    
    /// CHECK:
    pub mint_key: AccountInfo<'info>,
    #[account(mut)] 
    pub nft_info: Account<'info , NftInfo>,
    /// CHECK:
    #[account(mut)]
    pub from_ata: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub to_ata: AccountInfo<'info>, 
    /// CHECK:
    #[account(mut)]
    pub price_payer: AccountInfo<'info>,
    /// CHECK:
    pub nft_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    
}

#[derive(Accounts)]
pub struct RemoveNftFromSale<'info> {
    
    /// CHECK:
    pub mint_key: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub nft_info: Account<'info, NftInfo>,
    
}


#[derive(Accounts)]
pub struct AddBid<'info> {
    
    #[account(init, payer = nft_bidder, 
    space = 64+64+64,
    seeds = [b"config", mint_key.key().as_ref(), nft_bidder.key().as_ref()],
    bump
    )]
    pub nft_info: Account<'info, NftInfo>,
    /// CHECK:
    pub mint_key: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub nft_bidder: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct PlaceNftForTimedAuction<'info> {
    
    /// CHECK:
    pub mint_key: AccountInfo<'info>,
    #[account(mut)]
    pub nft_info: Account<'info, NftInfo>,
    /// CHECK:
    // Authority should be Signer
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
}



#[error_code]
pub enum ErrorCode{
    #[msg("Lower Amount")]
    LoweAmount, 
    #[msg("Invalid Argument")]
    InvalidArgument, 
    #[msg("Insufficient Balance ")]
    InsufficientBalance,
    #[msg("NFT is Already on Sale ")]
    NFTAlreadyOnSale,
    #[msg("Invalid Time for Auction ")]
    InvalidTime, 
    #[msg("NFT Not On Sale ")]
    NFTNotOnSale,
    #[msg("Less Bid Amount")]
    LessBidAmount,
    #[msg("Only Owner Has Access")]
    OnlyOwnerCan, 
}
