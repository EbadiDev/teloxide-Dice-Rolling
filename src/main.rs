use teloxide::prelude::*;
use teloxide::types::DiceEmoji;
use serde::Deserialize;
use teloxide_macros::BotCommands;
use std::fs;

#[derive(Deserialize)]
struct BotConfig {
    telegram_bot_token: String,
}

#[derive(Deserialize)]
struct Config {
    bot: BotConfig,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "throw a dice.")]
    Throw,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let config: Config = toml::from_str(&fs::read_to_string("config.toml").expect("Unable to read config file"))
        .expect("Unable to parse config file");

    let bot = Bot::new(config.bot.telegram_bot_token);
    Command::repl(bot, handle_command).await;
}

async fn handle_command(bot: Bot, message: Message, command: Command) -> ResponseResult<()> {
    match command {
        Command::Start => {
            bot.send_message(message.chat.id, "Welcome! Type /throw to throw a dice.").await?;
        },
        Command::Throw => {
            let dice = bot.send_dice(message.chat.id).emoji(DiceEmoji::Dice).await?;
            if let Some(dice_value) = dice.dice().map(|d| d.value) {
                bot.send_message(message.chat.id, format!("You rolled a {}. Roll again or type something else to stop.", dice_value)).await?;
            } else {
                bot.send_message(message.chat.id, "Failed to roll the dice.").await?;
            }
        },
        _ => {
            bot.send_message(message.chat.id, "Type /throw to throw a dice.").await?;
        },
    }
    Ok(())
}