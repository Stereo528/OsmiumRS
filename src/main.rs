use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::env;

use serenity::prelude::*;
use serenity::{
    async_trait,
    client::bridge::gateway::{GatewayIntents, ShardId, ShardManager},
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args,
        CommandGroup,
        CommandOptions,
        CommandResult,
        DispatchError,
        HelpOptions,
        Reason,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
};
//yes im aware none of these are used yet, i just imported all of them since i will need them at some point

//Shows the format for config.json for people to create one
//also allows the things being read from the config.json to be assigned the correct types
#[derive(Deserialize, Debug)]
struct Config {
    token: String,
    prefix: String,
}



//Serenity
#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}


#[tokio::main]
async fn main() {

    //json

    //No idea what this does, i just know its a way to read json files and be able to grab each key by itself
    let mut file = File::open("config.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    //actual assigning of keys to types
    let config: Config = serde_json::from_str(&buff).unwrap();  
    // if you do not call a key it will append "Config" to the start of the output
    // and otherwise be the whole json as a string. be careful when using this without keys


    
    //serenity stuff
        
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&config.prefix)) // set the bot's prefix to what is in the config
        .group(&GENERAL_GROUP);

    let mut client =
        Client::builder(&config.token).event_handler(Handler).framework(framework).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

