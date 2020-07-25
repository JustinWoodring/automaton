use crate::Config;
use crate::abilities::Ability;
use crate::pick_statement;
use serenity::prelude::*;
use serenity::model::prelude::*;

pub struct Capability;

impl Ability for Capability{
    fn guild_member_addition(&self, config: &Config, ctx: &Context, guild_id: &GuildId, new_member: &Member) -> bool{
        //If the server id matches the server id in config.
        if guild_id.as_u64() == &config.get_server_id(){

            //Send message to greet user.
            let _res = ChannelId(
                config.get_welcome_channel_id()
            ).say(
                &ctx,
                format!(
                    "Hello <@{}>! Are you a student?\nIf so then please type 'Yes, I am a student!'",
                    new_member.user_id()
                )
            );

            return true;
        }
        
        //If we don't return true assume false.
        false
    }
    
    fn message(&self, config: &Config, ctx: &Context, msg: &Message) -> bool{
        //Get message server.
        if let Some(guild_id) = msg.guild_id
        {
            //If server in config matches message's server.
            if guild_id.as_u64() == &config.get_server_id()
            {

                //Ask students if they are a student.
                if msg.content.to_lowercase() == "yes, i am a student!" ||
                    msg.content.to_lowercase() == "yes." ||
                    msg.content.to_lowercase() == "yes" ||
                    msg.content.to_lowercase() == "yes!"
                {
                    //Reply negatively if response is not enthusiastic.
                    if msg.content.to_lowercase() == "yes." ||
                        msg.content.to_lowercase() == "yes"
                    {
                        let _res = ChannelId(
                            config.get_welcome_channel_id()
                        ).say(
                            &ctx,
                            pick_statement(vec![
                                "*Someone* isn't as excited about this class as I am... Hold on one second.",
                                "You just couldn't humor me could you? I'll get you setup."
                            ])
                        );
                    }
                    //Reply positively.
                    else
                    {
                        let _res = ChannelId(
                            config.get_welcome_channel_id()
                        ).say(
                            &ctx,
                            pick_statement(vec![
                                "Wonderful! Allow me to get you setup.",
                                "Finally! Someone as excited as I am!"
                            ])
                        );
                    }


                    //See if they are still in the server???
                    if let Some(mut member) = msg.member(&ctx){

                        //Add role to member.
                        match member.add_role(&ctx, RoleId(730255796847509586))
                        {
                            Ok(_) => {
                                //Success.
                                let _res = ChannelId(
                                    config.get_welcome_channel_id()
                                ).say(
                                    &ctx,
                                    pick_statement(vec![
                                        "Ok, everything looks good! If you have need any help or have some questions ask an admin or let me know! :)",
                                        "Everything's all setup! Let me or an admin know if you need help! :)"
                                    ])
                                );
                            },
                            Err(_) => {
                                //Fail.
                                let _res = ChannelId(
                                    config.get_welcome_channel_id()
                                ).say(
                                    &ctx,
                                    pick_statement(vec![
                                        "Strange... I can't seem to work my space magic for some reason. <@730255796847509586>! I need some help here!"
                                    ])
                                );
                            }
                        }

                        return true;
                    }
                } 
                //I'm a teapot!
                else if msg.content.to_lowercase() == "no, i am a teapot!"
                {
                    if let Some(mut member) = msg.member(&ctx)
                    {
                        match member.add_role(&ctx, RoleId(732078892021579857))
                        {
                            Ok(_) => 
                            {
                                //Make them a teapot!
                                let _res = ChannelId(
                                        config.get_welcome_channel_id()
                                    ).say(
                                        &ctx,
                                        "It has been written and so it is! You shall stand as a lighthouse of the people."
                                    );

                                //Direct message them the order of space magic channel.
                                if let Ok(private) = msg.author.create_dm_channel(&ctx)
                                {
                                    let _res = private.say(
                                        &ctx,
                                        "Welcome to the Order of Space Magic!\nhttps://discord.gg/VhTRMrn"
                                    );
                                }
                            }
                            Err(_) => 
                            {
                                let _res = ChannelId(
                                    config.get_welcome_channel_id()
                                ).say(
                                    &ctx,
                                    "You are unworthy!"
                                );
                            }
                        }

                        return true;
                    }
                }
            }
        }

        return false;
    }
}