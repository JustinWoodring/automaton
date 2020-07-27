
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
            &format!(
                "{}{}",
                &config
                    .get_self_mention_regex_start(),
                "drop (.*).$"
            )
        )
            .unwrap();
        
        //Match and Capture.
        let caps = re.captures(&msg.content);
        
        //If successful.
        if let Some(caps) = caps 
        {
            
            //Assume they are not an admin and check.
            let mut is_admin = false;

            //Check if member.
            if let Some(member) = msg.clone().member
            {
                //Check if admin.
                for role in member.roles 
                {
                    if role == RoleId(config.get_admin_role_id())
                    {
                        //Update variable.
                        is_admin = true;
                    }
                }
                
                //Do stuff if user is an admin.
                if is_admin
                {
                    //Get the gif name.
                    if let Some(gif_name) = caps.get(1)
                    {
                        match gif_name.as_str()
                        {
                            "letsgo" => {let _res = msg.reply(ctx, "https://tenor.com/view/lets-go-ugh-kid-hyped-excited-gif-13907860");},
                            "trustme" => {let _res = msg.reply(ctx, "https://tenor.com/view/trust-me-chill-nbd-no-big-deal-iknow-what-im-doing-gif-5476814");},
                            "run" => {let _res = msg.reply(ctx, "https://tenor.com/view/explosion-action-bird-run-running-gif-4877919");},
                            "imout" => {let _res = msg.reply(ctx, "https://tenor.com/view/run-away-im-out-hopping-gif-15647210");},
                            "boom" => {let _res = msg.reply(ctx, "https://tenor.com/view/nuke-press-the-button-bomb-them-nuke-them-cat-gif-16361990");},
                            _ => {let _res = msg.reply(ctx, "https://tenor.com/view/confused-what-huh-gif-13526427");}
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
}
