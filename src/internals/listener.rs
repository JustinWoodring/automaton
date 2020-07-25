use crate::Config;

use serenity::CacheAndHttp;
use std::io::Read;
use serde_json::json;
use std::net::TcpListener;
use std::{thread, time};
use std::thread::JoinHandle;
use std::sync::Arc;

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
                        
                        //TODO
                        //Implement more advanced message parsing.


                        //Build message.
                        let map = json!({
                            "content": buffer,
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