use crate::Config;
use crate::pick_statement;
use super::parser;

use serenity::CacheAndHttp;
use std::io::Read;
use serde_json::json;
use std::net::TcpListener;
use std::{thread, time};
use std::thread::JoinHandle;
use std::sync::Arc;
use regex::Regex;
//use chrono::prelude::*;


pub struct Listener {
    config: Config,
    cache_and_http: Arc<CacheAndHttp>,
    handle: Option<JoinHandle<()>>
}

impl Listener {
    pub fn new(config: Config, cache_and_http: Arc<CacheAndHttp>) -> Self{
        Listener {
            config: config,
            cache_and_http: cache_and_http,
            handle: None
        }
    }

    pub fn run(&mut self){
        let config = self.config.clone();
        let cache_and_http = self.cache_and_http.clone();

        //Spin up thread.
        let thread = thread::spawn(move || 
        {

            //Bind port in config
            let listener = TcpListener
                ::bind(
                    format!(
                        "{}:{}",
                        config.get_listening_address(),
                        config.get_listening_port()
                    )
                )
                    .unwrap();
            
            //Foreach connection open a stream and read contents.
            for stream in listener.incoming() 
            {

                if let Ok(mut stream) = stream
                {

                    //Set timeout.log
                    let _result = stream
                        .set_read_timeout(
                            Some(
                                time::Duration::from_millis(500)
                            )
                        );


                    //Copy contents to buffer.
                    let mut buffer = String::new();
                    if let Ok(_) = stream.read_to_string(&mut buffer)
                    {
                        //Statement to send.
                        let mut content = String::new();

                        //Break request down and extract JSON.
                        let re = Regex::new(r"(?s)(\A.*?)(\{.*)").unwrap();
                        
                        //Match.
                        let caps = re.captures(&buffer);

                        //If successful.
                        if let Some(caps) = caps
                        {

                            //See if we captured the JSON.
                            if let Some(json) = caps.get(2)
                            {

                                //Parse the JSON.
                                if let Some(event) = parser::github::parse(json.as_str()) 
                                {
                                    match event 
                                    {
                                        parser::github::GithubEvents::Fork(repo_url, sender) => 
                                        {
                                            let option1 = format!("@everyone! I bear dreadful news. {} has forked their repo. The forked repo was: {}.", sender.as_str(), repo_url.as_str());
                                            let option2 = format!("@everyone. Please impart to these dilettantes the art of git. {} has forked their repo, {}.", sender.as_str(), repo_url.as_str());
                                            content = pick_statement(vec![
                                                &option1,
                                                &option2
                                            ]);
                                        },
                                        parser::github::GithubEvents::Push(repo_url, sender, forced) => 
                                        {
                                            if forced 
                                            {
                                                let option1 = format!("@everyone. {}'s repo has been obliterated by a force push. If there is anything left you can find it here: {}.", sender.as_str(), repo_url.as_str());
                                                let option2 = format!("@everyone. {} probably requires assistance restoring their repo, {}, to which they have just force pushed...", sender.as_str(), repo_url.as_str());
                                                content = pick_statement(vec![
                                                    &option1,
                                                    &option2
                                                ]);
                                            } 
                                            else 
                                            {
                                                let option1 = format!("This is seemingly unimportant, but {} has pushed to their repo, {}.", sender.as_str(),  repo_url.as_str());
                                                let option2 = format!("I have received notice that {} just pushed to their repo, {}.", sender.as_str(), repo_url.as_str());
                                                let option3 = format!("{} just pushed to their repo, {}.", sender.as_str(), repo_url.as_str());
                                                let option4 = format!("I just got a push notice. {} has pushed to their repo, {}.", sender.as_str(), repo_url.as_str());
                                                let option5 = format!("{} has pushed to their repo, {}. You are aware that I write these for posterity's sake right?", sender.as_str(), repo_url.as_str());
                                                let option6 = format!("{} made a push to their repo, {}. Let's go!", sender.as_str(), repo_url.as_str());
                                                let option7 = format!("{} has pushed to their repo, {}. If you find these notices troublesome I suppose you could just edit my source code.", sender.as_str(), repo_url.as_str());
                                                content = pick_statement(vec![
                                                    &option1,
                                                    &option2,
                                                    &option3,
                                                    &option4,
                                                    &option5,
                                                    &option6,
                                                    &option7
                                                ]);
                                            }
                                        },
                                        parser::github::GithubEvents::PullRequest(repo_url, sender) => 
                                        {
                                            let option1 = format!("@everyone. A rather peculiar event just occurred. {} opened a pull request in their repo: {}.", sender.as_str(), repo_url.as_str());
                                            let option2 = format!("@everyone. Hmm. Curious. {} just opened a pull request. Perhaps we should investigate, {}.", sender.as_str(), repo_url.as_str());
                                            content = pick_statement(vec![
                                                &option1,
                                                &option2
                                            ]);
                                        },
                                    }
                                }
                            }
                        }

                        //Attach timestamp.
                        //let utc: DateTime<Utc> = Utc::now(); 

                        //Build message.
                        let map = json!({
                            "content": content,
                            "tts": false,
                        });
                        

                        //Send the message.
                        let _result = cache_and_http
                            .http
                            .send_message(
                                config
                                    .get_carnival_channel_id(),
                                &map
                            );
                    }
                }
                

                //Sleep to let discord bot take over.
                thread::sleep(time::Duration::from_millis(300));
            }
        });


        //Add join handle.
        self.handle = Some(thread);

    }
}
