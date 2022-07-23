use poise::serenity_prelude::Mentionable;

use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn test(
    ctx: Context<'_>,
    // #[description = "desc"] opt: String,
) -> Result<(), Error> {
    {
        let data = ctx.data();
        let mut raid = data.raid.lock().unwrap();

        let user_id = ctx.author().id;

        dbg!(raid.add_user(user_id, 5));
        dbg!(raid.get_user_completed_codes(user_id).len());
        dbg!(raid.opt_user(user_id, 5));
        dbg!(raid.get_user_codes(user_id));
        dbg!(raid.submit_codes(user_id));
        dbg!(raid.change_user_rate(user_id, 10));
        dbg!(raid.submit_codes(user_id));
        dbg!(raid.deopt_user(user_id));
        dbg!(raid.get_user_completed_codes(user_id).len());
        dbg!(raid.get_user_codes(user_id));
    }

    // dbg!(user_id);
    ctx.say("hi").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let codes: Vec<String>;
    let completed_codes: usize;
    {
        let data = ctx.data();
        let mut raid = data.raid.lock().unwrap();

        let user_id = ctx.author().id;

        dbg!(raid.add_user(user_id, 5));
        dbg!(raid.get_user_completed_codes(user_id).len());
        dbg!(raid.opt_user(user_id, 5));
        // say the user joined the raid then list their codes all in one message
        codes = raid.get_user_codes(user_id);
        completed_codes = raid.get_user_completed_codes(user_id).len();
        dbg!(&codes);
        dbg!(&completed_codes);
    }
    let codes_str = codes.join("\n");
    ctx.say(&format!(
        "{} joined the raid.\nThey have {} completed codes their codes are:\n{}",
        ctx.author().mention(),
        completed_codes,
        codes_str
    ))
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn submit(ctx: Context<'_>) -> Result<(), Error> {
    let new_codes: Vec<String>;
    let completed_codes: usize;
    {
        let data = ctx.data();
        let mut raid = data.raid.lock().unwrap();

        let user_id = ctx.author().id;

        new_codes = raid.submit_codes(user_id);
        dbg!(&new_codes);
        completed_codes = raid.get_user_completed_codes(user_id).len();
        dbg!(&completed_codes);
    }
    let new_codes_str = new_codes.join("\n");
    ctx.say(&format!(
        "{} submitted their codes.\nThey have completed {} codes. Their new codes are:\n{}",
        ctx.author().mention(),
        completed_codes,
        new_codes_str
    ))
    .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let completed_codes: usize;
    {
        let data = ctx.data();
        let mut raid = data.raid.lock().unwrap();

        let user_id = ctx.author().id;

        dbg!(raid.deopt_user(user_id));
        completed_codes = raid.get_user_completed_codes(user_id).len();
        dbg!(&completed_codes);
    }
    ctx.say(&format!(
        "{} left the raid. They had completed {} codes.",
        ctx.author().mention(),
        completed_codes
    ))
    .await?;
    Ok(())
}
