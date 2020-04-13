use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
pub fn roasted(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let message = crate::utils::find_relative_content(ctx, msg, args)?;
    if let Ok(url) = crate::imgflip::generate_image_url(None, Some(&message), "122616222") {
        let _ = msg
            .channel_id
            .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author));
    }
    let _ = msg.delete(&ctx);
    Ok(())
}