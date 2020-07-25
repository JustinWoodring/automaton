use serenity::prelude::*;
use serenity::model::prelude::*;
use rand::Rng;
pub use internals::instance::Instance;
pub use internals::config::Config;
use abilities::AbilityHandler;
use abilities::EventEnum;

mod internals;
mod abilities;

pub struct HandlerWrapper {
    handler: AbilityHandler,
    config: Config
}

#[allow(clippy::if_same_then_else)]
impl EventHandler for HandlerWrapper {
    fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member){
        self.handler.handle(EventEnum::GuildMemberAddEvent(ctx, guild_id, new_member))
    }
    fn message(&self, ctx: Context, msg: Message) {
        self.handler.handle(EventEnum::MessageCreateEvent(ctx, msg))
    }
    fn ready(&self, ctx: Context, _: Ready) {
        ctx.reset_presence();
    }
}

fn pick_statement(var : Vec<&str>) -> String{
    if var.len() > 1 {
        let mut rng = rand::thread_rng();
        let string : String = var.get(rng.gen_range(0, var.len())).unwrap().to_string();
        string.clone()
    }else{
        let string : String = var.get(0).unwrap().to_string();
        string.clone()
    }
}