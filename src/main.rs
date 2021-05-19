mod templates;

use serenity::{
    async_trait,
    model::{channel::{Message, Reaction}, gateway::Ready, guild::Member, id::*},
    client::bridge::gateway::GatewayIntents,
    prelude::*,
};

use sailfish::TemplateOnce;

use templates::*;

fn log_result<T>(res: Result<T, impl std::fmt::Debug>) {
    if let Err(e) = res {
        println!("Error: {:?}", e);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("New message");
        if msg.content == "!ping" {
            log_result(msg.channel_id.say(&ctx.http, "Pong!").await);
        }

        // Example-bot channel ID = 843635272310325278
        if msg.channel_id == ChannelId(843635272310325278) {
            if msg.content == "!dm_me" {
                let member = msg.member(&ctx).await;
                if let Ok(m) = member {
                    log_result(m.user.direct_message(&ctx, |send| {
                        send.content(Dm01 {
                            username: &m.user.name
                        }.render_once().unwrap())
                    }).await)
                }
            }
        }
    }

    async fn reaction_add(&self, ctx: Context, new_reaction: Reaction) {
        println!("New reaction");
        // Welcome message ID = 844374407338131505
        if new_reaction.message_id == MessageId(844374407338131505) {
            // Unwrap is panic-free here because the above MessageId is guaranteed to be part of a guild.
            let member = new_reaction.guild_id.unwrap().member(&ctx, new_reaction.user_id.unwrap()).await;
            if let Ok(mut m) = member {
                log_result(m.add_role(&ctx.http, 844374637437911060).await);
            }
        }
    }

    async fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, _new_member: Member) {
        println!("New member");
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = dotenv::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .intents(GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
