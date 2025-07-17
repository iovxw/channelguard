use teloxide::prelude::*;
use teloxide::types::ChatMemberKind;
use teloxide::{
    Bot,
    adaptors::throttle::{Limits, Throttle},
    requests::RequesterExt,
};
use unicode_segmentation::UnicodeSegmentation;

// Workaround for iterating [async fn]
macro_rules! unroll_for {
    ($item:ident in [$($items:ident),+] $body:block) => {
        $(
            let $item = $items;
            $body
        )*
    };
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::new("TOKEN").throttle(Limits::default());
    let channel = ChatId(-100);
    let group = ChatId(-100);
    let admin = ChatId(100);

    teloxide::repl(bot, move |bot: Throttle<Bot>, msg: Message| async move {
        unroll_for!(detector in [non_channel_member, only_one_emoji, ad_sticker] {
            if detector(&bot, &msg, channel, group).await? {
                bot.forward_message(admin, msg.chat.id, msg.id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                return Ok(());
            }
        });
        Ok(())
    })
    .await;
}

async fn non_channel_member(
    bot: &Throttle<Bot>,
    msg: &Message,
    channel: ChatId,
    group: ChatId,
) -> Result<bool, teloxide::RequestError> {
    // if msg is from the group and sender is not the linked channel
    if msg.chat.id == group && msg.sender_chat.is_none() && let Some(user) = &msg.from {
            if let teloxide::types::ChatMember {
                kind: ChatMemberKind::Left,
                ..
            } = bot.get_chat_member(channel, user.id).await?
            {
                return Ok(true);
            }
    }
    Ok(false)
}

async fn only_one_emoji(
    _bot: &Throttle<Bot>,
    msg: &Message,
    _channel: ChatId,
    _group: ChatId,
) -> Result<bool, teloxide::RequestError> {
    if let Some(text) = msg.text() {
        let is_one_emoji = text.graphemes(true).count() == 1 && emojis::get(text).is_some();
        let english_first_name = msg.from.as_ref().map_or(false, |user| {
            user.first_name.chars().all(|c| c.is_ascii_alphabetic())
        });
        let emoji_last_name = msg.from.as_ref().map_or(false, |user| {
            user.last_name
                .as_ref()
                .map_or(false, |last_name| emojis::get(last_name).is_some())
        });
        return Ok(is_one_emoji && english_first_name && emoji_last_name);
    }
    Ok(false)
}

async fn ad_sticker(
    bot: &Throttle<Bot>,
    msg: &Message,
    _channel: ChatId,
    _group: ChatId,
) -> Result<bool, teloxide::RequestError> {
    if let Some(teloxide::types::Sticker {
        set_name: Some(set_name),
        ..
    }) = msg.sticker()
    {
        let sticker_set = bot.get_sticker_set(set_name).await?;
        if sticker_set.title.contains("@") {
            for keyword in ["赌博", "博彩", "福利"] {
                if sticker_set.title.contains(keyword) {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}
