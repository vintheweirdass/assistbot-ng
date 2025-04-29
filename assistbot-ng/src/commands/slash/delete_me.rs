use serenity::{all::{ChannelId, Context, CreateCommand, GetMessages, Interaction, Message, MessageId, UserId}, async_trait};
use crate::commands::util::slash::{CirmResult, ScCommon, SlashCommand};

const MSG_LIMIT:u8 = 30;
pub struct DeleteMe {}
impl DeleteMe {}
#[async_trait]
impl<'a> SlashCommand for DeleteMe {
    async fn run(&self, ctx: &Context, interaction: &Interaction, common: &ScCommon) -> CirmResult {
        let command = &common.command;
        let user_id = &command.user.id;
        
        let channel_id = command.channel.as_ref().ok_or(common.error("This channel dosent found"))?.id;

        let channel_exist = channel_id.to_channel(&ctx.http).await
            .ok().ok_or(common.error("The channel dosent exist"))?;
        let _convert_to_guild_channel = channel_exist.guild()
            .ok_or(common.error("Channel is not a guild channel"))?;
        let perms = interaction.app_permissions().ok_or(common.error("Im not in guild channel"))?;
        if !perms.manage_messages() {
            return Err(common.error("I can't manage messages!\n> For admins: We don't explicitly request
            to manage messages since its considered suspicious for these type of bots. You can enable
            manually by setting the `AssistBot` role and enable the `Manage Messages`"))
        }
        let _ = delete_user_messages(ctx, channel_id, user_id).await
            .ok().ok_or(common.error("Failed to delete messages"));

        return Ok(common.reply("Done! started a cleaning thread to delete your messages!"))
    }
    fn register(&self) -> CreateCommand {
        return CreateCommand::new("delete_me")
            .description("(GUILD ONLY): delete all of your messages in this channel")
    }
}


async fn retrieve_messages(
    ctx: &Context,
    channel_id: ChannelId,
    before: Option<MessageId>,
) -> Result<Vec<Message>, serenity::Error> {
    let builder = match before {
        Some(before_id) => GetMessages::new().before(before_id).limit(100),
        None => GetMessages::new().limit(MSG_LIMIT),
    };

    channel_id.messages(&ctx.http, builder).await
}

async fn delete_user_messages(
    ctx: &Context,
    channel_id: ChannelId,
    user_id: &UserId,
) -> Result<(), serenity::Error> {
    let mut deleted = 0;
    let messages= retrieve_messages(ctx, channel_id, None).await?;
    if &messages.len() > &MSG_LIMIT.into() {
        return Ok(());
    }
    while deleted<MSG_LIMIT {
        // Filter messages by user ID
        let user_messages: Vec<&Message> = messages
            .iter()
            .filter(|msg| &msg.author.id == user_id)
            .collect();

        // Delete the filtered messages
        for message in user_messages {
            message.delete(&ctx.http).await?;
            deleted += 1;
        }
    }
    Ok(())
}
