use log::info;
use teloxide::{prelude::*, utils::command::BotCommands};
use rand::Rng;
use tokio::time::{sleep};
use std::time::Duration;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Inciando RojãoBot(rojao_rust_bot)!");

    let bot = Bot::from_env();

    Command::repl(bot, handler).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Comandos que podem ser executados")]
enum Command {
    #[command(description = "Acende o rojão")]
    Acende,
    Start
}

fn get_total_shots() -> u8 {
    let mut generator = rand::thread_rng();
    return generator.gen_range(1..10);
}

fn get_total_explosions() -> u8 {
    let mut generator = rand::thread_rng();
    return generator.gen_range(1..6);
}

async fn handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Acende => {
            info!("Rojão disparado...");
            let mut shots = get_total_shots();

            while shots > 0 {
                let explosions = get_total_explosions();
                let text = format!("pra ").repeat(explosions.into());
                bot.send_message(msg.chat.id, text).await?;

                shots -= 1;
            }

            sleep(Duration::from_millis(2000)).await;
            bot.send_message(msg.chat.id, "POW").await?;
        }

        Command::Start => {
            bot.send_message(msg.chat.id, format!("Use o comando /acende para disparar um rojão")).await?;
        }
    };

    Ok(())
}