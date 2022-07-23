mod commands;
mod handler;
mod setup;
// use commands::*;

use handler::CodeRaidHandler;
use poise::serenity_prelude as serenity;
use std::io::Write;
use std::sync::Mutex;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {
    raid: Mutex<CodeRaidHandler>,
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let data = match setup::setup().await {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {}", e);
            print!("Press enter to exit...");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            std::process::exit(1);
        }
    };

    let options = poise::FrameworkOptions {
        commands: vec![
            register(),
            commands::code_raid::test(),
            commands::code_raid::join(),
            commands::code_raid::submit(),
            commands::code_raid::leave(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            case_insensitive_commands: true,
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    let framework = poise::Framework::build()
        .options(options)
        .token(data.0)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    raid: Mutex::new(CodeRaidHandler::new(data.1)),
                })
            })
        });

    framework.run().await.unwrap();
}
