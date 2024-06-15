use anchor_lang::prelude::*;

declare_id!("sxdW9mkroQdRiYsSoLS8jyoqKXEYuPRjXF5enErjo8G");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn adding_task(ctx: Context<AddingTask>, description: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); // Getting the current timestamp
        
        if description.chars().count() > 400 {
            return Err(ErrorCode::TextTooLong.into());
        }
        
        task.author = *author.key;
        task.is_achieved = false;
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.description = description;
        Ok(())

    }

    pub fn updating_task(ctx: Context<UpdatingTask>, is_achieved: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); // Getting the current timestamp
        
        task.author = *author.key;
        task.is_achieved = is_achieved;
        task.updated_at = clock.unix_timestamp;
        Ok(())
       
    }

    pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The `author` account
        let clock = Clock::get().unwrap(); // Getting the current timestamp
        
        task.author = *author.key;
        task.is_achieved = true;
        task.updated_at = clock.unix_timestamp;
        Ok(())
       
    }




}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdatingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeletingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}


#[account]
pub struct Task {
    pub author: Pubkey,  // The account that owns the task
    pub is_achieved: bool,   // Whether the task is completed or not
    pub description: String,    // The description of the task
    pub created_at: i64, // The timestamp when the task was created
    pub updated_at: i64, // The timestamp when the task was last updated
}

const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BOOL_LENGTH: usize = 1;
const TEXT_LENGTH: usize = 4 + 400 * 4; // 400 chars
const TIMESTAMP_LENGTH: usize = 8;

impl Task {
    const LEN: usize = DISCRIMINATOR + // discriminator
        PUBLIC_KEY_LENGTH + // author
        BOOL_LENGTH + // is_achieved
        TEXT_LENGTH +  // description
        TIMESTAMP_LENGTH + // created_at
        TIMESTAMP_LENGTH; // updated_at
}


#[error_code]
pub enum ErrorCode {
    #[msg("The description is too long")]
    TextTooLong,
}