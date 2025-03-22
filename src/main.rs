mod commands;
mod settings;
use commands::util::common::EmbedFromSettings;
use commands::util::slash::CirmResult;
use commands::util::CommandError;
use commands::util::{Common, slash::ScCommon, slash::SlashCommand};
use commands::MESSAGE_BASED_COMMANDS;
use commands::SLASH_COMMANDS;
use std::sync::Arc;
use serenity::all::{Command, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::*;
use shuttle_runtime::{SecretStore, Error as shuttle_error};
use tracing::{warn, info};
use reqwest::Client as HttpClient;

struct Bot<'a> {
    common: Common,
    slash_commands:Arc<Mutex<Vec<(Command, &'a Box<dyn SlashCommand>)>>>
}

#[async_trait]
impl <'a> EventHandler for Bot <'static> {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(ref command) = interaction {
            let name = command.data.name.as_str();
            let mut msg:CirmResult = Some(Err(CommandError::Default("Command dosent found".to_string())));
            let command_map = self.slash_commands.lock().await;
            let commands = command_map;
            for (info, command_proc) in commands.iter() {
                    if name != info.name {
                        continue
                    }
                    msg = command_proc.run(&ctx, &interaction, &ScCommon {
                        command:command.clone(), http_client:self.common.http_client.clone()
                    }).await;
                    break;
            }


            if let Some(Err(err)) = msg {
                let embed: CreateEmbed = match err {
                    CommandError::Default(v)=>{
                        CreateEmbed::new_from_settings().title("Error").description(v)
                    },
                    CommandError::Argument(name, msg)=>{
                        CreateEmbed::new_from_settings().title("Error").description(msg)
                        .footer(CreateEmbedFooter::new(format!("Caused by argument '{name}'")))
                    },
                };
                let data = CreateInteractionResponseMessage::new().embed(embed);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    warn!("Cannot respond to slash command: {why}");
                }
            } else if let Some(Ok(content)) = msg {
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    warn!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        if settings::DISABLE_MESSAGE_BASED_COMMANDS {
            return;
        }
        let allow_only_on = settings::ONLY_ALLOW_MESSAGE_BASED_COMMANDS_ON_CHANNELS;
        if allow_only_on.len()>0 && !allow_only_on.contains(&&msg.channel_id.to_string().as_str()) {
            return;
        }
        let content = msg.content.clone();
        let content_lower = content.clone().to_lowercase();
        if !content_lower.starts_with(settings::MESSAGE_BASED_COMMANDS_PREFIX) {
            return;
        }

        for command_raw in MESSAGE_BASED_COMMANDS.iter() {
            if command_raw.is_none() {
                continue
            }
            let command = command_raw.as_ref().unwrap();
            if content_lower.starts_with(&format!("{}{}", settings::MESSAGE_BASED_COMMANDS_PREFIX, command.name())) {
                command.run(&ctx, &msg, &self.common).await;
                break;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        let mut command_map: Vec<(Command, &Box<dyn SlashCommand>)> = vec![];
        let activity = &*settings::SET_ACTIVITY_DATA_WHEN_CONNECTED;
        let status = settings::SET_ONLINE_STATUS_WHEN_CONNECTED;

        ctx.set_presence(Some(activity.clone()), status);
        // Register all commands and store their mapping
        for (_, command_opt) in SLASH_COMMANDS.iter().enumerate() {
            if let Some(command) = command_opt {
                let gc_raw = Command::create_global_command(
                    &ctx.http, 
                    command.register()
                ).await;
                
                if let Ok(gc) = gc_raw {
                    // Store the mapping from command name to (Command, index)
                    command_map.push((gc, command));
                } else if let Err(err) = gc_raw {
                    warn!("Error adding command {}", err.to_string());
                }
            }
        }
        
        // Update the command map using the mutex
        let mut map = self.slash_commands.lock().await;
        *map = command_map;
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets.get("ASSISTBOT_DISCORD_TOKEN").ok_or_else(|| {
        shuttle_error::BindPanic("'ASSISTBOT_DISCORD_TOKEN' was not found".to_owned())
    })?;

    // Set gateway intents
    let mut intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_INTEGRATIONS;
    let intents_from_settings = &settings::INTENTS;
    if !settings::DISABLE_MESSAGE_BASED_COMMANDS && !intents_from_settings.contains(&GatewayIntents::GUILD_MESSAGES) {
        return Err(shuttle_error::BindPanic(
            "Add GatewayIntents::GUILD_MESSAGES in settings::INTENTS since message-based commands weren't disabled"
                .to_owned(),
        ));
    }
    for each in intents_from_settings {
        intents |= *each;
    }
    let client = Client::builder(&token, intents)
        .event_handler(Bot {
            common: Common {http_client:HttpClient::new()},
            slash_commands: Arc::new(Mutex::new(vec![]))
        })
        .await;

    if client.is_err() {
        return Err(shuttle_error::BuildPanic("Failed to make Serenity client".to_owned()));
    }

    Ok(client.unwrap().into())
}

