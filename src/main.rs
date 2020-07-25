use automaton::{Instance, Config};


fn main(){
    //Load config.
    let config = Config::read_from_file("config.yaml").expect("Couldn't read file.");    
    
    //Make a new instance of automaton.
    let mut instance = Instance::new(config);

    //Kick instance.
    instance.run();
}