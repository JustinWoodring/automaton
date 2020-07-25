use crate::Config;
use crate::abilities::Ability;
use crate::pick_statement;
use serenity::prelude::*;
use serenity::model::prelude::*;
use regex::Regex;



pub struct Capability;

impl Ability for Capability{
    fn message(&self, config: &Config, ctx: &Context, msg: &Message) -> bool{

        //Build regex.
        let re = Regex::new(
            &config.get_self_mention_regex()
        )
            .unwrap();

        //Match and capture.
        let caps = re.captures(
            &msg.content
        );
        
        //If successful.
        if let Some(_caps) = caps 
        {
            let _res = msg.reply(
                ctx,
                pick_statement(vec![
"
  Hello, I can see that you mentioned me. That said I don't really know what you want. Here are the services I provide:\n
  - I can delete everything in a channel that isn't pinned (if you are an admin). To issue 'at' me and say 'kill it with fire.'\n
  - I can provide anyone with high-quality curated documentation on several subjects. To issue 'at' me and say 'I need resources on (subject here)'\n
  - I also welcome new students.
"
                ])
            );

            return true;
        }

        false
    }
}