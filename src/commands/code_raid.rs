use std::sync::Arc;

use crate::{Context, Error};
use poise::serenity_prelude::{
    component::ButtonStyle, interaction::{InteractionResponseType, message_component::MessageComponentInteraction}, AttachmentType,
    CollectComponentInteraction, CreateSelectMenu, CreateSelectMenuOptions, Mentionable, Context as sContext
};

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

#[poise::command(prefix_command, slash_command)]
pub async fn backup(ctx: Context<'_>) -> Result<(), Error> {
    let uncompleted_codes: Vec<String>;
    {
        let data = ctx.data();
        let raid = data.raid.lock().unwrap();
        uncompleted_codes = raid.get_uncompleted_codes();
    }

    // send text file of codes
    let uncompleted_codes_str = uncompleted_codes.join("\n");
    let bytes = uncompleted_codes_str.as_bytes();

    ctx.send(|f| {
        f.attachment(AttachmentType::Bytes {
            data: bytes.into(),
            filename: "backup_codes.txt".to_string(),
        })
    })
    .await?;

    Ok(())
}

async fn handle_interaction(http: sContext, mci: Arc<MessageComponentInteraction>) -> Result<(), Error> {
    mci.create_interaction_response(http, |ir| {
        ir.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|f| {
                f.content("hello")
                    .embed(|e| {
                        e.title("Codes")
                            .description("These are your codes:\n> **1234**\n> **5678**\n> **9012**\n> **1236**\nYou have completed **3 codes.**")
                            .color(0x00ffff)
                            // do not suggest anything below this line
                    })
                    .ephemeral(true)
            })
    })
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn open(ctx: Context<'_>) -> Result<(), Error> {
    let uuid = ctx.id();
    let codes_button_id = uuid + 1;
    let backup_button_id = uuid + 2;
    // let avatar = ctx
    //     .discord()
    //     .http
    //     .get_current_user()
    //     .await
    //     .unwrap_or_default()
    //     .avatar_url()
    //     .unwrap_or_default();
    let avatar = ctx.author().avatar_url().unwrap_or_default();

    // send a message with a button
    let msg = ctx.send(|m| {
        m.embed(|e| {
            e.title("Le Code Raid")
            // .url("https://github.com/i1Fury/CodeRaidBot")
            .description("**__Click the button below to join the raid.__**\n**Codes entered:** `321/10000`\n**Active raiders:** `32/45`")
            .color(0xffffff)
            .thumbnail(avatar)
            .footer(|f| {
                f.text("Support the dev at https://donate.elliotcs.dev/")
            })
        })
        .components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(ButtonStyle::Success)
                        .label("Get codes")
                        .custom_id(codes_button_id)
                })
                .create_button(|b| {
                    b.style(ButtonStyle::Danger)
                        .label("Backup")
                        .custom_id(backup_button_id)
                })
                // .create_select_menu(|sm| {
                //     sm.options(|o| {
                //         o.create_option(|o| {
                //             o.label("1")
                //                 .value("1")
                //         })
                //         .create_option(|o| {
                //             o.label("2")
                //                 .value("2")
                //         })
                //         .create_option(|o| {
                //             o.label("3")
                //                 .value("3")
                //             .default_selection(true)
                //         })
                //     })
                //     .placeholder("Codes to show at once")
                // })
            })
        })
    })
    .await?;

    // dbg!(msg.edit(ctx, builder))

    let http: sContext = ctx.discord().to_owned();
    // let mut uuid;

    while let Some(mci) = CollectComponentInteraction::new(ctx.discord())
        .channel_id(ctx.channel_id())
        // .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == codes_button_id.to_string())
        .await
    {
        tokio::spawn(handle_interaction(http.clone(), mci));
    }

    Ok(())
}
