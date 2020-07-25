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
                "kill it with fire."
            )
        )
            .unwrap();
        
        //Match and Capture.
        let caps = re.captures(&msg.content);
        
        //If successful.
        if let Some(_caps) = caps 
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
                    
                    //Positive reply.
                    let _res = msg.reply(
                        ctx,
                        pick_statement(vec![
                            "I will purge this channel with flames.",
                            "I will drown this channel in flames.",
                            "I shall inflame the channel."
                        ])
                    );

                    //Loop through all messages in batches of 100 and delete.
                    loop
                    {

                        //Assume we don't need to break.
                        let mut need_to_break = false;

                        //For use in the loop.
                        let msg2 = msg.clone();

                        //Load a batch of messages.
                        let messages = msg
                            .channel_id
                            .messages(
                                ctx.clone(),
                                move |retriever| 
                                {
                                    retriever.before(msg2.id).limit(100)
                                }
                            );

                        //Loop through batch of messages if there are messages.
                        if let Ok(messages) = messages
                        {
                            if messages.is_empty()
                            {
                                break;
                            }

                            //Loop and delete. If there is an error break.
                            for message in messages
                            {
                                if !message.pinned
                                {
                                    match message.delete(ctx) 
                                    {
                                        Ok(_) => {},
                                        Err(_) => {
                                            need_to_break = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            
                            //Break on error.
                            if need_to_break
                            {
                                break;
                            }
                        }
                        //No more messages? break.
                        else
                        {
                            break;
                        }
                    }

                    //Delete last message.
                    let _res = msg.delete(ctx);
                }
                else
                {
                    //Repond negatively.
                    let _res = msg.reply(
                        ctx,
                        pick_statement(vec![
                            "I shan't.",
                            "Whom do you think you are?",
                            "Certainly not!"
                        ])
                    );
                }
            }
            //It worked cool.
            return true;
        }

        //It no work.
        false
    }
}