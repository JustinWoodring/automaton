use json;

pub enum GithubEvents {
    Fork(String, String),
    PullRequest(String, String),
    Push(String, String, bool)
}

pub fn parse(input : &str) -> Option<GithubEvents>{
    
    //Check Fork
    if let Some(event) = check_fork(input)
    {
        return Some(event)
    }

    //Check Pull Request
    if let Some(event) = check_pull_request(input)
    {
        return Some(event)
    }

    //Check Push
    if let Some(event) = check_pull_request(input)
    {
        return Some(event)
    }

    None
}


//Github Events Parse Functions
fn check_fork(input : &str) -> Option<GithubEvents>{
    if let Ok(object) = json::parse(input)
    {
        //Check for fork.
        if !object["forkee"].is_null()
        {

            //Check to make sure repo exists.
            if object["repository"].is_null()
            {
                return None
            }

            //Check to make sure sender exists.
            if object["sender"].is_null()
            {
                return None
            }

            
            //Now check to see if the fields we are looking for exist.
            if object["repository"]["html_url"].is_null()
            {
                return None
            }

            if object["sender"]["login"].is_null()
            {
                return None
            }

            //And that the fields are strings.
            if !object["repository"]["html_url"].is_string()
            {
                return None
            }

            if !object["sender"]["login"].is_string()
            {
                return None
            }

            return Some(GithubEvents::Fork(
                object["repository"]["html_url"].as_str().unwrap().to_string(),
                object["sender"]["login"].as_str().unwrap().to_string()
            ));

        }
    }
    None
}


fn check_pull_request(input : &str) -> Option<GithubEvents>{
    if let Ok(object) = json::parse(input)
    {
        //Check for pull request.
        if !object["pull_request"].is_null() && !object["action"].is_null()
        {
            //Check for pull request opened?
            if !(object["action"].is_string())
            {
                return None
            }

            if !(object["action"].as_str().unwrap() == "opened"){
                return None
            }


            //Check to make sure repo exists.
            if object["repository"].is_null()
            {
                return None
            }

            //Check to make sure sender exists.
            if object["sender"].is_null()
            {
                return None
            }

            
            //Now check to see if the fields we are looking for exist.
            if object["repository"]["html_url"].is_null()
            {
                return None
            }

            if object["sender"]["login"].is_null()
            {
                return None
            }


            //And that the fields are strings.
            if !object["repository"]["html_url"].is_string()
            {
                return None
            }

            if !object["sender"]["login"].is_string()
            {
                return None
            }

            return Some(GithubEvents::PullRequest(
                object["repository"]["html_url"].as_str().unwrap().to_string(),
                object["sender"]["login"].as_str().unwrap().to_string()
            ));

        }
    }
    None
}


fn check_push(input : &str) -> Option<GithubEvents>{
    if let Ok(object) = json::parse(input)
    {
        //Check for pusher.
        if !object["pusher"].is_null()
        {

            //Check to make sure repo exists.
            if !object["repository"].is_null()
            {
                return None
            }

            //Check to make sure sender exists.
            if !object["sender"].is_null()
            {
                return None
            }

            //Check to make sure forced exists.
            if !object["forced"].is_null()
            {
                return None
            }

            //And that it is a bool.
            if object["forced"].is_boolean()
            {
                return None
            }

            
            //Now check to see if the fields we are looking for exist.
            if !object["repository"]["html_url"].is_null()
            {
                return None
            }

            if !object["sender"]["login"].is_null()
            {
                return None
            }


            //And that the fields are strings.
            if !object["repository"]["html_url"].is_string()
            {
                return None
            }

            if !object["sender"]["login"].is_string()
            {
                return None
            }


            return Some(GithubEvents::Push(
                object["repository"]["html_url"].as_str().unwrap().to_string(),
                object["sender"]["login"].as_str().unwrap().to_string(),
                object["forced"].as_bool().unwrap()
            ));

        }
    }
    None
}