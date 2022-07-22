use poise::serenity_prelude as serenity;

pub mod setup;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dbg!();
    let data = match setup::setup().await {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    dbg!();
    // let data = setup().await.unwrap();
    dbg!(&data.0);
    dbg!(&data.1.len());

    let options = poise::FrameworkOptions {
        commands: vec![age(), register()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            case_insensitive_commands: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let framework = poise::Framework::build()
        .options(options)
        .token(data.0)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
