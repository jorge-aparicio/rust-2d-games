use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct Story {
    pub story_name: String,
    pub scenes: Vec<NamedScene>,
}


#[derive(Serialize, Deserialize)]
pub struct NamedScene {
    pub scene_name: String,
    pub scene: Scene,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
    pub name: String,
    pub message: String,
    pub response_message: String,
    pub responses: Vec<Response>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    pub response: String, // ?????
    pub goto: String,
}

pub fn parse_story() -> Result<(HashMap<String, Scene>)> {
    // Some JSON input data as a &str. Maybe this comes from the user.


    // TODO change to text reader to parse story from file
    let data = r#"{
    "scene_name": "intro", 
    "scene" :
    {

      "name": "Nemo",
      "message": "Hi! My name is Nemo. You all may know me from the rather popular Disney Pixar movie Finding Nemo. It's a little story about how my dad goes looking for me in the Great Barrier Reef when I was just a wee little clown fish. TL;DR is I wanted to go out exploring the great unknown because my dad, Marlin, was kinda an overprotective clown fish (mom died when I was just an egg). So I did what every angsty 6-year-old would do. Swim off by myself and explore the mysterious reef! I met so many cool people on my journey, but was honestly super happy my dad found me. Now I'm 16 years old, all grown up, and ready for my next adventure! Dad isn't as protective anymore, but he still wants me to be safe. Help me find the great treasure of the barrier reef! (I heard it gives you superpowers)...",
      "response_message": "So will you help me?",
      "responses": [
    {
    "response":  "Yes",
    "goto": "startadventure"
    },
    {
    "response":  "No",
    "goto": "ending1"
    }
    
    ]
    }
  }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let scenes: Vec<NamedScene> = serde_json::from_str(data)?;

    let mut scene_map: HashMap<String,Scene> = HashMap::new();
    
    scenes.iter().for_each(|s| {
      scene_map.insert(s.scene_name.clone(), s.scene.clone());

    });
    

    Ok((scene_map))
}

