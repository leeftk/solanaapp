use anchor_lang::prelude::*;

declare_id!("GVi4HHgQ4eSfiidepQtEmNBwLgz5kQsUiwzW6iUKuvSy");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.cid_count = 0;
        Ok(())
    }

    // pub fn print_cid(ctx: Context<StartStuffOff>, cidid: usize) -> Result<()> {
    //     let base_account = &mut ctx.accounts.base_account;
    //     print!("{:?}", base_account.cid_list[cidid].cid_link.to_string());
    //  Ok(())
    // }

    // The function now accepts a cid_link param from the user.
    pub fn add_gif(ctx: Context<AddGif>, cid_link: String, track_title: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            track_title: track_title.to_string(),
            cid_link: cid_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // Add it to the cid_list vector.
        base_account.cid_list.push(item);
        base_account.cid_count += 1;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub track_title: String,
    pub cid_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub cid_count: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub cid_list: Vec<ItemStruct>,
}

