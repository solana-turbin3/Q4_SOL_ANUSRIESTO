use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("4zfjRX3nU9g2TGpGpdNmRLcjAwqpwPtBMGUBcKaPNjpz");

#[program]
pub mod vault_prog {
    use anchor_lang::Bumps;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx:Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.deposit(amount);
        Ok(())
    }

    pub fn withdraw(ctx:Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.withdraw(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer=user,
        space=VaultState::INIT_SPACE,
        seeds=[b"state",user.key().as_ref()],
        bump
    )]
    pub state:Account<'info,VaultState>,
    #[account(
        seeds=[b"vault",state.key().as_ref()],
        bump,
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>

}

impl <'info> Initialize<'info>{
    pub fn initialize(&mut self,bumps:InitializeBumps)->Result<()>{
            self.state.vault_bump=bumps.vault;
            self.state.state_bump=bumps.state;

            Ok(())

    }
}

#[derive(Accounts)]
pub struct Payment <'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        seeds=[b"state",user.key().as_ref()],
        bump
    )]
    pub state:Account<'info,VaultState>,
    #[account(
        seeds=[b"vault",state.key().as_ref()],
        bump,
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>

}

impl<'info> Payment<'info>{
    pub fn deposit(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.system_program.to_account_info();
        let cpi_accounts=Transfer{
            from:self.user.to_account_info(),
            to:self.vault.to_account_info(),
        };
        let cpi_ctx=CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)
    }
    pub fn withdraw(&mut self,amount:u64,bump:&[u8])->Result<()>{
        let user_seed=[&[self.user.key.as_ref(),&[self.Vaultstate];
        let cpi_program=self.system_program.to_account_info();
        let cpi_accounts=Transfer{
            from:self.vault.to_account_info(),
            to:self.user.to_account_info(),
        };
        let cpi_ctx=CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&user_seed[..]]);
        transfer(cpi_ctx, amount)
    }
}


#[account]
//#[derive(InitSpace)]
pub struct VaultState{
    pub vault_bump:u8,
    pub state_bump:u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}
