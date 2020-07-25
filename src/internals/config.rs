use std::fs;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Config{
    //Setup
    listening_address: String,
    listening_port: u64,

    //Discord Setup
    token: String,
    self_id: u64,
    server_id: u64,

    //Discord Roles
    self_role_id: u64,
    admin_role_id: u64,

    //Discord Channels
    welcome_channel_id: u64,
    carnival_channel_id: u64,
}

impl Config{
    pub fn read_from_file(address : &str) -> Result<Config, Box<dyn std::error::Error + 'static>>{
        let string = fs::read_to_string(address)?;
        let config_string : Result<Config, serde_yaml::Error> = serde_yaml::from_str(&string); 
        let config = config_string?;
        Ok(config)
    }
    
    //Setup
    pub fn get_listening_address(&self) -> String{
        return self.listening_address.clone();
    }

    pub fn get_listening_port(&self) -> u64{
        return self.listening_port.clone();
    }

    //Discord Setyp
    pub fn get_token(&self) -> String{
        return self.token.clone();
    }
    
    pub fn get_self_id(&self) -> u64{
        return self.self_id.clone();
    }
    
    pub fn get_server_id(&self) -> u64{
        return self.server_id.clone();
    }


    //Roles
    pub fn get_self_role_id(&self) -> u64{
        return self.self_role_id.clone();
    }
    
    pub fn get_admin_role_id(&self) -> u64{
        return self.admin_role_id.clone();
    }
    

    //Channels
    pub fn get_welcome_channel_id(&self) -> u64{
        return self.welcome_channel_id.clone();
    }

    pub fn get_carnival_channel_id(&self) -> u64{
        return self.carnival_channel_id.clone();
    }
    

    //Regexes
    pub fn get_self_mention_regex(&self) -> String{
        return format!(r"<@(?:!{}|&{})>\s*", self.get_self_id(), self.get_self_role_id()).clone();
    }
    
    pub fn get_self_mention_regex_start(&self) -> String{
        return format!(r"^<@(?:!{}|&{})>\s*", self.get_self_id(), self.get_self_role_id()).clone();
    }
}