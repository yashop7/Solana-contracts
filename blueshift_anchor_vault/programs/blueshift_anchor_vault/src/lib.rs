use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultExists);

        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );
        ctx.accounts.deposit_to_lamports(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        require!(
            ctx.accounts.vault.lamports() > 0,
            VaultError::VaultNotExists
        );
        let bump = ctx.bumps.vault;
        ctx.accounts.withdraw_close_vault(bump)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> VaultAction<'info> {
    pub fn deposit_to_lamports(&self, amount: u64) -> Result<()> {
        // we will deposit the amount from Signer into Vault
        anchor_lang::system_program::transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: self.signer.to_account_info(),
                    to: self.vault.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn withdraw_close_vault(&mut self, bump: u8) -> Result<()> {
        let signer_key = self.signer.key();
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", signer_key.as_ref(), &[bump]]];

        // Transfer all lamports from vault back to signer
        let vault_lamports = self.vault.to_account_info().lamports();
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: self.vault.to_account_info(),
                    to: self.signer.to_account_info(),
                },
                signer_seeds,
            ),
            vault_lamports,
        )?;

        Ok(())
    }
}

#[error_code]
pub enum VaultError {
    // error enum
    #[msg["Not Authorized"]]
    NotAuthorized,
    #[msg["Vault already Exist"]]
    VaultExists,
    #[msg["Vault doesn't Exist"]]
    VaultNotExists,
    #[msg["Invalid Amount"]]
    InvalidAmount,
}
