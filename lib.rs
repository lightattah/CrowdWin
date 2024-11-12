use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("HSvNFaKcvduW8ikrjoCpFkKrieQ2XnVisRgyXUPPspkm");

#[program]
mod contest_app {
    use super::*;

    // Create a new contest
    pub fn create_contest(
        ctx: Context<CreateContest>,
        title: String,
        description: String,
        deadline: i64,
    ) -> Result<()> {
        let contest = &mut ctx.accounts.contest;
        contest.title = title;
        contest.description = description;
        contest.deadline = deadline;
        contest.prize_pool = 0;
        contest.owner = ctx.accounts.owner.key();
        contest.is_closed = false;
        Ok(())
    }

    // Fund the contest and allocate votes
    pub fn fund_contest(ctx: Context<FundContest>, amount: u64) -> Result<()> {
        let contest = &mut ctx.accounts.contest;
        let vote_account = &mut ctx.accounts.vote_account;

        // Transfer funds to the contest's prize pool
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.funder.to_account_info(),
                to: contest.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;

        contest.prize_pool += amount;
        vote_account.contest = contest.key();
        vote_account.funder = ctx.accounts.funder.key();
        vote_account.allocated_votes = amount / 1_000_000; // Example: 1 SOL = 1 vote
        vote_account.used_votes = 0;
        Ok(())
    }

    // Submit an entry to a contest
    pub fn submit_entry(ctx: Context<SubmitEntry>, content_link: String) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        let contest = &ctx.accounts.contest;
        require!(!contest.is_closed, ContestClosed);

        entry.contest = contest.key();
        entry.creator = ctx.accounts.participant.key();
        entry.content_link = content_link;
        entry.votes = 0;
        Ok(())
    }

    // Vote for an entry
    pub fn vote_for_entry(ctx: Context<VoteForEntry>, votes: u64) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        let vote_account = &mut ctx.accounts.vote_account;

        require!(
            entry.contest == vote_account.contest,
            ContestMismatch
        );
        require!(
            vote_account.funder == ctx.accounts.voter.key(),
            UnauthorizedVoter
        );
        require!(
            vote_account.allocated_votes >= votes,
            InsufficientVotes
        );

        entry.votes += votes;
        vote_account.used_votes += votes;
        vote_account.allocated_votes -= votes;
        Ok(())
    }

    // Close the contest and distribute the prize to the winner
    pub fn close_contest(ctx: Context<CloseContest>) -> Result<()> {
        let contest = &mut ctx.accounts.contest;
        require!(contest.owner == ctx.accounts.owner.key(), Unauthorized);

        contest.is_closed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateContest<'info> {
    #[account(init, payer = owner, space = 8 + 256)]
    pub contest: Account<'info, Contest>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundContest<'info> {
    #[account(mut)]
    pub contest: Account<'info, Contest>,
    #[account(init, payer = funder, space = 8 + 64)]
    pub vote_account: Account<'info, Vote>,
    #[account(mut)]
    pub funder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitEntry<'info> {
    #[account(mut)]
    pub contest: Account<'info, Contest>,
    #[account(init, payer = participant, space = 8 + 256)]
    pub entry: Account<'info, Entry>,
    #[account(mut)]
    pub participant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteForEntry<'info> {
    #[account(mut)]
    pub entry: Account<'info, Entry>,
    #[account(mut)]
    pub vote_account: Account<'info, Vote>,
    #[account(mut)]
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseContest<'info> {
    #[account(mut)]
    pub contest: Account<'info, Contest>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Contest {
    pub title: String,
    pub description: String,
    pub deadline: i64,
    pub prize_pool: u64,
    pub owner: Pubkey,
    pub is_closed: bool,
}

#[account]
pub struct Entry {
    pub contest: Pubkey,
    pub creator: Pubkey,
    pub content_link: String,
    pub votes: u64,
}

#[account]
pub struct Vote {
    pub contest: Pubkey,
    pub funder: Pubkey,
    pub allocated_votes: u64,
    pub used_votes: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The contest is closed.")]
    ContestClosed,
    #[msg("Contest ID does not match.")]
    ContestMismatch,
    #[msg("Unauthorized voter.")]
    UnauthorizedVoter,
    #[msg("Insufficient votes.")]
    InsufficientVotes,
    #[msg("Unauthorized.")]
    Unauthorized,
}
