use teloxide::prelude::*;
use teloxide::types::{ChatMemberKind, MessageKind};
use teloxide::{
    Bot,
    adaptors::throttle::{Limits, Throttle},
    requests::RequesterExt,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::new(env!("TELOXIDE_TOKEN")).throttle(Limits::default());
    let channel = ChatId(-1001076595214);
    let group = ChatId(-1002127274702);

    teloxide::repl(bot, move |bot: Throttle<Bot>, msg: Message| async move {
        dbg!("new msg");
        if msg.chat.id == group && msg.sender_chat.is_none() {
            if let Some(user) = &msg.from {
                match bot.get_chat_member(channel, user.id).await {
                    Ok(m) if m.kind == ChatMemberKind::Left => {
                        bot.delete_message(group, msg.id).await?;
                        if let MessageKind::Common(mut msg) = msg.kind {
                            msg.reply_to_message = None;
                            dbg!(msg);
                        } else {
                            dbg!(msg);
                        }
                    }
                    Err(teloxide::RequestError::Api(teloxide::ApiError::Unknown(e))) => {
                        dbg!(&msg, e);
                    }
                    Err(teloxide::RequestError::Api(e)) => {
                        dbg!(msg, e);
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    })
    .await;
}
