use serde::{Deserialize, Serialize};
use serde_json::Result;



#[derive(Serialize, Deserialize)]
struct NamedScene {
    scene_name: String,
    scene: Scene,
}

#[derive(Serialize, Deserialize)]
struct Scene {
    name: String,
    message: String,
    response_message: String,
    responses: Vec<Response>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    response: String, // ?????
    goto: String,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
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
    let p: NamedScene = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("{}: Do you want to come along? {}", p.scene_name, p.scene.name);

    Ok(())
}

fn main() {
	typed_example().unwrap();
}