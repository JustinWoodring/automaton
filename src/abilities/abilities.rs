use crate::Config;
use serenity::prelude::*;
use serenity::model::prelude::*;


pub struct AbilityHandler {
    abilities: Vec<Box<dyn Ability>>,
    config: Config
}

impl AbilityHandler{
    
    pub fn new(config: Config) -> Self{
        //Create a new AbilityHandler here.
        let mut this = AbilityHandler {
            abilities: Vec::new(),
            config
        };

        //Add Abilities here
        this.add_ability(Box::new(super::delete::Capability));
        this.add_ability(Box::new(super::student::Capability));
        this.add_ability(Box::new(super::resource::Capability));
        //Add most default Ability here.
        this.add_ability(Box::new(super::mention::Capability));
        //Return Ability Handler.
        this
    }

    pub fn handle(&self, event: EventEnum){
        let event = std::sync::Arc::new(event);
        for ability in &self.abilities{
            match event.clone().as_ref() {
                EventEnum::GuildMemberAddEvent(ctx, guild_id, member) => {
                    if ability.guild_member_addition(&self.config, ctx, guild_id, member)==true {
                        break;
                    }
                }
                EventEnum::MessageCreateEvent(ctx, message) => {
                    if ability.message(&self.config, ctx, message)==true {
                        break;
                    }
                }
            }
        }
    }

    fn add_ability(&mut self, ability: Box<dyn Ability>){
        self.abilities.push(ability);
    }
}


//Update the following if you need to add more supported types of events.
pub enum EventEnum{
    GuildMemberAddEvent(Context, GuildId, Member),
    MessageCreateEvent(Context, Message),
}

pub trait Ability: Sync + Send{
    fn guild_member_addition(&self, _config: &Config, _ctx: &Context, _guild_id: &GuildId, _new_member: &Member) -> bool {
        false
    }
    
    fn message(&self, _config: &Config, _ctx: &Context, _message: &Message) -> bool{
        false
    }
}