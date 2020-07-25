use crate::Config;
use crate::abilities::Ability;
use crate::pick_statement;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serde::{Serialize, Deserialize};
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
                    "I need resources on (.+)\\.$"
                )
            )
                .unwrap();

        //Match and Capture.      
        let caps = re.captures(&msg.content);

        //If successful.
        if let Some(caps) = caps
        {

            //Tell users we are typing because this takes a while.
            let _res = msg
                .channel_id
                .broadcast_typing(ctx);

            //Access captures.
            if let Some(_string) = caps.get(0)
            {

                //Categories we are going to download to.
                let mut categories = Vec::new();
                
                //Try to download the categories.
                if let Ok(categories_string) = reqwest
                    ::blocking
                    ::get("https://raw.githubusercontent.com/Booglejr/osmwebsite/master/_data/categories.yml")
                {   
                    //Read the text in body of the reponse.       
                    if let Ok(categories_string) = categories_string.text()
                    {
                        //Try to deserialize the result into a struct.
                        let categories_string : 
                            Result
                            <
                                Vec<Category>,
                                serde_yaml::Error
                            >
                            = serde_yaml::from_str(&categories_string); 
                        
                        //If this is ok unpack into categories.
                        if let Ok(categories_string) = categories_string
                        {
                            categories = categories_string;
                        }
                    }
                }
                
                //Get the requested resource.
                if let Some(resource) = caps.get(1)
                {
                    let mut resource_request = String::new();
                    let mut matches = false;

                    //Figure out if the requested resource matches a category.
                    for category in &categories 
                    {
                        if resource.as_str().to_lowercase() == category.title.to_lowercase() {
                            resource_request=category.tagname.clone();
                            matches = true;
                            break;
                        }
                    }
                    
                    //If it does.
                    if matches {
                        
                        //Entries we intend to display.
                        let mut entries_to_display : Vec<Entry> = Vec::new();
                        
                        //Vec we will store entries in.
                        let mut entries = Vec::new();
                    
                        //Try to download the entries..
                        if let Ok(entries_string) = reqwest
                            ::blocking
                            ::get("https://raw.githubusercontent.com/Booglejr/osmwebsite/master/_data/entries.yml")
                        {
                            //Read the body text. 
                            if let Ok(entries_string) = entries_string.text()
                            {  
                                //Deserialize into entries string.
                                let entries_string : Result<Vec<Entry>, serde_yaml::Error> = serde_yaml::from_str(&entries_string); 
                                
                                //If its ok then try to unpack into entries.
                                if let Ok(entries_string) = entries_string
                                {
                                    entries = entries_string;
                                }
                            }
                        }       
                        
                        //Find out which entries should be displayed and move those to the display list.
                        for entry in entries{

                            //Should we display this entry?
                            let mut display_entry = false;

                            //Let's loop through the tags and see if any of them match the category.
                            for tag in &entry.tags{
                                if &resource_request == tag{
                                    display_entry = true;
                                    break;
                                }
                            }
                            
                            //Ok if we should display this lets push it into entries to display.
                            if display_entry == true{
                                let new_entry = Entry {
                                    ..entry
                                };
                                
                                //Push it into the other vec.
                                entries_to_display.push(new_entry);
                            }
                        }

                        //Positive reply.
                        let _res = msg.reply(
                            ctx,
                            pick_statement(vec![
                                "Here you go:\n\n -",
                                "As requested:\n\n -",
                                "Anything else I can help you with?:\n\n -"
                            ])
                        );
                        
                        
                        //Store long response into one message.
                        let mut response = String::new();

                        //Loop through entries and format/append each entry to response.
                        for entry in entries_to_display
                        {
                            response+=format!("\n\n**{}**\n  Description: {}\n  Url: {}\n", entry.name, entry.description, entry.url).as_str();
                        }

                        //Throw nice formatty thing on there.
                        response+="-";
                        
                        //List entries to user.
                        let _res = msg.channel_id.say(
                            &ctx,
                            response
                        );
                    }
                    //If category doesn't match.
                    else
                    {
                        //Negative reply.
                        let _res = msg.reply(
                            ctx,
                            pick_statement(vec![
                                "So, I wasn't able to find what you were looking for, below is a list of items I can recommend resources on:", 
                                "I'm sorry but I don't have anything, I can provide you with resources on the following items:", 
                                "Hmm... I can't seem to find that. I only have resources on the items below."
                            ])
                        );

                        //Store response into one message.
                        let mut response = String::new();

                        //Loop through categories and append each to the message.
                        for category in &categories{
                            response+=format!(
                                "\n- {}",
                                category.title
                            )
                                .as_str();
                        }

                        //Send the message.
                        let _res = msg.channel_id.say(
                            &ctx,
                            response
                        );
                    }

                    return true
                }
            }
        }

        false
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Category{
    title: String,
    tagname: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Entry{
    name: String,
    tags: Vec<String>,
    url: String,
    description: String,
    submitter: String
}