use std::env;

use dotenv::dotenv;
use log::{info, warn};
use serenity::async_trait;
use serenity::model::gateway::GatewayIntents;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(
        &self,
        _ctx: serenity::prelude::Context,
        _old: Option<VoiceState>,
        _new: VoiceState,
    ) {
        let observing_channel_id = env::var("CHANNEL").expect("Observing channel is not found");
        let new_member = _new.member.unwrap();
        let new_member_name = new_member.user.name;

        match _new.channel_id {
            Some(new_channel_id) => {
                let channel_id = new_channel_id.to_string();
                if observing_channel_id == channel_id {
                    println!("join {}", new_member_name);
                }
            }
            None => match _old {
                Some(old) => {
                    if observing_channel_id == old.channel_id.unwrap().to_string() {
                        println!("leave {}", new_member_name);
                    }
                }
                None => warn!("こまこまのこまり"),
            },
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Discord token is not found");

    // Intents are a bitflag, bitwise operations can be used to dictate which intents to use
    let intents =
        GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS;
    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
