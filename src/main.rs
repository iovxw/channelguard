use teloxide::prelude::*;
use teloxide::types::ChatMemberKind;
use teloxide::{
    Bot,
    adaptors::throttle::{Limits, Throttle},
    requests::RequesterExt,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::new("TOKEN").throttle(Limits::default());
    let channel = ChatId(-100);
    let group = ChatId(-100);

    teloxide::repl(bot, move |bot: Throttle<Bot>, msg: Message| async move {
        if msg.chat.id == group && msg.sender_chat.is_none() {
            if let Some(user) = &msg.from {
                if matches!(bot.get_chat_member(channel, user.id).await, Ok(m) if m.kind == ChatMemberKind::Left) {
                    bot.delete_message(group, msg.id).await?;
                    dbg!(msg);
                }
            }
        }
        Ok(())
    })
    .await;
}
