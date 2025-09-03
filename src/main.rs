use serde::Deserialize;
use reqwest;

use teloxide::{prelude::*, utils::command:: BotCommands};

#[tokio::main]
async fn main(){
    pretty_env_logger::init();
    log::info!("Starting command bot!!");
    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

#[derive(Deserialize, Debug)]
struct Feriado{
    date: String,
    name: String,
    #[serde(rename(deserialize="type"))]
    tipo: String}

async fn get_feriados(ano: String) -> Result<(), reqwest::Error>{
    let request_url = format!("https://brasilapi.com.br/api/feriados/v1/{ano}");
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();
    let feriados: Vec<Feriado> = client.get(request_url).send().await?.json().await?;
}    

#[derive(BotCommands, Clone)]
#[command(rename_rule="lowercase", description="These commands are supported:")]
enum Command{
    #[command(description="Display this text.")]
    Help,
    #[command(description="Handle a username.")]
    Username(String),
    #[command(description="Handle a username and an age", parse_with="split")]
    UsernameAndAge{username: String, age: u8},
    #[command(description="Pega os feriados do ano")]
    Feriados(String)
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()>{
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {bot.send_message(msg.chat.id, format!("Your username is @{username}")).await?}
        Command::UsernameAndAge { username, age} => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}, and your age is {age}.")).await?
        }
        Command::Feriados(feriado) => {
            bot.send_message(msg.chat.id, get_feriados(feriado)).await?
        }
    };
    Ok(())
}