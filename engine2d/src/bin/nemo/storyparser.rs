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

pub fn parse_story() -> Result<(Story)> {
    // Some JSON input data as a &str. Maybe this comes from the user.


    // TODO change to text reader to parse story from file
    let data = include_str!("nemo_test.json");

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let story: Story = serde_json::from_str(data)?;

    
    //let mut scene_map: HashMap<String,Scene> = HashMap::new();
    
    //scenes.iter().for_each(|s| {
      //scene_map.insert(s.scene_name.clone(), s.scene.clone());

   //});
    

    Ok((story))
}

