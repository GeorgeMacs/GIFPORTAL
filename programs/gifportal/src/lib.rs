use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("BtfUF3J9YgdRitEQJ6gNQ5KJboEBAKTpaPxqygvp92Hk");

#[program]
pub mod gifportal {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct{
            gif_link: gif_link.toString()
            user_address: *user.to_account_info().key
        }

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }


}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer=user, space=9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddGif<'info, BaseAccount> {
    #[account (mut)]
    pub base_account: Account <'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link String,
    pub useraddress: Pubkey
}
#[account]
pub struct BaseAccount{
    pub total_gifs: u64,
    pub gif_list: Vec ItemStruct
}

