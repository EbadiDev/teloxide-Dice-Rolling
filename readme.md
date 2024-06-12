### Usage

- Users can start the bot with the `/start` command and receive a welcome message.
- Users can roll a dice by sending the `/throw` command.
- After rolling the dice, the bot will display the value of the roll and prompt the user to either roll again or type something else to stop.
- If the user types anything other than `/throw`, the bot will prompt them to type `/throw` to continue rolling.


### Installation

1. Clone the repository:

2. Install the required dependencies by running:
    `cargo build`
3. Create a `config.toml` file in the project root directory with your Telegram bot token:

```toml
[bot]
telegram_bot_token = "your_bot_token_here"

Replace "your_bot_token_here" with the actual token provided by the BotFather when you created your Telegram bot.

To run the bot, execute the following command:
`cargo run`