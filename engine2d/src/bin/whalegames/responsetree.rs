/* A tree structure for the whale games. Each tree node is composed of a message the player can read, a list of responses from which the player can make a decision,
and the children representing the different pathways in the story the player can take based on their reponses.*/
#[derive(Clone, PartialEq)]
pub struct ListTreeNode {
    pub message: String,
    pub text_index: usize,
    // response index should correspond with child index
    pub responses: Vec<String>,

    // current selected response by user
    pub response_index: usize,
    //pub child_type: TextType,
    pub children: Vec<ListTreeNode>,
}

impl ListTreeNode {
    pub fn new(message: String, responses: Vec<String>, children: Vec<ListTreeNode>) -> Self {
        Self {
            message,
            text_index: 0,
            responses,
            response_index: 0,
            children,
        }
    }
    pub fn add(&mut self, message: String, responses: Vec<String>, children: Vec<ListTreeNode>) {
        self.children.push(ListTreeNode {
            message,
            text_index: 0,
            responses,
            response_index: 0,
            children,
        })
    }
    pub fn next(&mut self, index: usize) {
        self.message = self.children[index].message.clone();
        self.text_index = 0;
        self.responses = self.children[index].responses.clone();
        self.response_index = 0;
        self.children = self.children[index].children.clone();
    }
}

pub struct Pointer {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}
