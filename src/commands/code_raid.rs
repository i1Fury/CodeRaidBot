use crate::{Context, Error};


#[poise::command(prefix_command, slash_command)]
pub async fn start(
    ctx: Context<'_>,
    // #[description = "desc"] opt: String,
) -> Result<(), Error> {
    dbg!(ctx.author().id);
    ctx.say("hi").await?;
    Ok(())
}