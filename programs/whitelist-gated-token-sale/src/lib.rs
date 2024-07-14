use anchor_lang::prelude::*;
// use anchor_lang::solana_program::system_instruction;
use std::collections::HashMap;

declare_id!("2Bd6iBjiRP3ste4ooqNXjBMAQ1v4QLQhAmhZJanL66QG");

#[program]
mod whitelist_gated_token_sale {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        admin: Pubkey,
        token_to_sol_price: u32,
        token_scale: u8,
        max_tokens_per_buyer: u64,
        tokens_available_for_sale: u64,
    ) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelist;
        whitelist.admin = admin;
        whitelist.pda_accounts = Vec::new();

        let token_sale_details: &mut TokenSaleDetails = &mut ctx.accounts.token_sale_details;

        token_sale_details.admin = admin;
        token_sale_details.max_tokens_per_buyer = max_tokens_per_buyer;
        token_sale_details.tokens_available_for_sale = tokens_available_for_sale;
        token_sale_details.amount_of_tokens_bought_by_buyer = HashMap::new();

        // Set initial token price and scale
        ctx.accounts.token_to_sol_price.token_to_sol_price = token_to_sol_price;
        ctx.accounts.token_to_sol_price.scale = token_scale;

        Ok(())
    }

    pub fn add_address_to_whitelist(
        ctx: Context<AddAccountToWhitelist>,
        address_to_whitelist: Pubkey,
    ) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelist;
        require!(
            ctx.accounts.admin.key() == whitelist.admin,
            CustomError::Unauthorized
        );

        let current_pda = whitelist.get_current_pda();
        let no_of_addresses_in_current_pda = ctx
            .accounts
            .new_whitelist_pda
            .get_no_of_addresses_in_pda(current_pda);

        match no_of_addresses_in_current_pda {
            0..=9 => {
                ctx.accounts
                    .new_whitelist_pda
                    .add_address_to_whitelist(address_to_whitelist)?;
            }
            10 => {
                let new_key = Pubkey::new_unique();
                ctx.accounts
                    .new_whitelist_pda
                    .create_new_pda_and_add_address(new_key, address_to_whitelist)?;
                whitelist.add_new_pda(new_key)?;
            }
            _ => {
                return Err(CustomError::PDAFull.into());
            }
        }

        Ok(())
    }

    pub fn get_whitelist_admin(ctx: Context<GetWhitelistAdmin>) -> Result<Pubkey> {
        let admin_account = ctx.accounts.admin.key();
        Ok(admin_account.clone())
    }

    pub fn buy_token_from_sale(ctx: Context<BuyTokenFromSale>, amount_to_buy: u64) -> Result<()> {
        let buyer: &Signer = &ctx.accounts.buyer;
        let mut is_buyer_whitelisted: bool = false;
        let _whitelist = &ctx.accounts.whitelist;
        let is_sale_active: &mut TokenSaleDetails = &mut ctx.accounts.token_sale_details;

        for pda in &ctx.accounts.whitelist_pdas.pda_accounts {
            if pda.contains(&buyer.key()) {
                is_buyer_whitelisted = true;
                break;
            }
        }

        require!(is_buyer_whitelisted, CustomError::NotWhitelisted);
        require!(is_sale_active.active == true, CustomError::SaleInactive);

        match is_sale_active
            .amount_of_tokens_bought_by_buyer
            .get(&buyer.key())
        {
            Some(tokens_bought) => {
                require!(
                    (tokens_bought + amount_to_buy) <= is_sale_active.max_tokens_per_buyer,
                    CustomError::AmountExceedsMax
                );
            }
            None => {
                require!(
                    amount_to_buy < is_sale_active.max_tokens_per_buyer,
                    CustomError::PrevCurrentAmountExceedsMax
                );
            }
        }

        // Token Sale Logic

        // Transfer DOL to Admin
        let _token_scale: u64 = is_sale_active.token_to_sol_price.scale as u64;
        let _token_price: u64 = is_sale_active.token_to_sol_price.token_to_sol_price as u64;
        // let sol_cost_of_tokens_to_buy = 10u64.pow(9) * amount_to_buy * token_price / token_scale;

        // let sol_to_admin_ix: Instruction = system_instruction::transfer(
        //     &buyer.key(),
        //     &is_sale_active.admin,
        //     sol_cost_of_tokens_to_buy,
        // );

        // anchor_lang::solana_program::program::invoke_signed(
        //     &sol_to_admin_ix,
        //     &[
        //         from_account.to_account_info(),
        //         to_account.clone(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        //     &[],
        // )?;

        //

        let tokens_bought = is_sale_active
            .amount_of_tokens_bought_by_buyer
            .get(&buyer.key())
            .unwrap_or(&0);
        is_sale_active
            .amount_of_tokens_bought_by_buyer
            .insert(buyer.key(), tokens_bought + amount_to_buy);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 4)]
    pub token_to_sol_price: Account<'info, TokenToSolPrice>,

    #[account(init, payer = admin, space = 8 + 32 + 4 + 32 * 10)]
    pub whitelist: Account<'info, WhiteList>,

    #[account(init, payer = admin, space = 8 + 32 + 4 + 32 * 10)]
    pub whitelist_pdas: Account<'info, WhiteListPDA>,

    #[account(init, payer = admin, space = 1 + 8 + 8 + 32)]
    pub token_sale_details: Account<'info, TokenSaleDetails>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct WhiteList {
    pub pda_accounts: Vec<Pubkey>,
    pub admin: Pubkey,
}

impl WhiteList {
    fn get_current_pda(&self) -> Pubkey {
        self.pda_accounts
            .last()
            .expect("Empty Accounts Vector")
            .clone()
    }

    fn add_new_pda(&mut self, new_pda: Pubkey) -> Result<()> {
        self.pda_accounts.push(new_pda);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyTokenFromSale<'info> {
    #[account(mut)]
    buyer: Signer<'info>,

    #[account(mut)]
    pub whitelist: Account<'info, WhiteList>,

    #[account(mut)]
    pub whitelist_pdas: Account<'info, WhiteListPDA>,

    #[account(mut)]
    pub token_sale_details: Account<'info, TokenSaleDetails>,
}

#[account]
pub struct TokenSaleDetails {
    active: bool,
    tokens_available_for_sale: u64,
    max_tokens_per_buyer: u64,
    admin: Pubkey,
    amount_of_tokens_bought_by_buyer: HashMap<Pubkey, u64>,
    token_to_sol_price: TokenToSolPrice, // token_to_sol_price: Account<'info, TokenToSolPrice>
}

pub struct TransferSOL {}
#[account]
pub struct WhiteListPDA {
    pub pda_accounts: Vec<Vec<Pubkey>>,
}

impl WhiteListPDA {
    fn get_no_of_addresses_in_pda(&self, pda: Pubkey) -> usize {
        self.pda_accounts
            .iter()
            .find(|&pdas| pdas.contains(&pda))
            .map_or(0, |pdas| pdas.len())
    }

    pub fn add_address_to_whitelist(&mut self, address_to_whitelist: Pubkey) -> Result<()> {
        let last_pda = self
            .pda_accounts
            .last_mut()
            .ok_or(CustomError::PDAListEmpty)?;
        last_pda.push(address_to_whitelist);
        Ok(())
    }

    pub fn create_new_pda_and_add_address(
        &mut self,
        new_key: Pubkey,
        address_to_add: Pubkey,
    ) -> Result<()> {
        self.pda_accounts.push(vec![new_key, address_to_add]);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddAccountToWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub whitelist: Account<'info, WhiteList>,

    #[account(init_if_needed, payer = admin, space = 8 + 4 + 32 * 10)]
    pub new_whitelist_pda: Account<'info, WhiteListPDA>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetWhitelistAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[account]
pub struct TokenToSolPrice {
    pub token_to_sol_price: u32,
    pub scale: u8,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("PDA is full and cannot add more addresses")]
    PDAFull,
    #[msg("PDA List is empty")]
    PDAListEmpty,
    #[msg("Not whitelisted")]
    NotWhitelisted,
    #[msg("Sale Not in Progress")]
    SaleInactive,
    #[msg("Amount exceeds Max. Allocation")]
    AmountExceedsMax,
    #[msg("Previous Amount Bought + Current Amount exceeds Max. Allocation")]
    PrevCurrentAmountExceedsMax,
}
