use crate::{HandlerWrapper, abilities::AbilityHandler, Config};

use serenity::Client;



pub struct Instance {
    handler_wrapper: Option<HandlerWrapper>
}


impl Instance {
    pub fn new(config: Config) -> Self{
        Instance {
            handler_wrapper: Some(HandlerWrapper {
                handler: AbilityHandler::new(config.clone()),
                config: config.clone()
            })
        }
    }

    pub fn run(&mut self){
        let config_for_listener = self.handler_wrapper.as_ref().unwrap().config.clone();

        //Create the client.
        let mut client = Client::new(
            self
                .handler_wrapper.as_ref()
                .unwrap()
                .config
                .get_token(),

            self
                .handler_wrapper
                .take()
                .unwrap()
            )
                .expect("Could not create new client.");

        //Create the listener for listening to webhooks.internal
        let mut listener = super::listener::Listener::new(
            config_for_listener,
            client.cache_and_http.clone()
        );

        //Start the listener.
        listener.run();

        //Start the client we just created.
        client.start().expect("Could not start client.");
    }
}