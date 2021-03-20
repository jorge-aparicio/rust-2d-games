
pub enum TextType{
    Message,
    Response(i16)
}
pub struct ListTreeNode {
    pub text: String,
    pub text_type: TextType, 
    pub children: Vec<ListTreeNode>,
}



impl ListTreeNode {
    pub fn new(text: String, text_type: TextType, children: Vec<ListTreeNode>)-> Self{
        Self{
            text: text,
            text_type: text_type,
            children: children,
        }
    }
    pub fn add(&mut self, text: String, text_type: TextType, children: Vec<ListTreeNode>){
        self.children.push(ListTreeNode{
            text: text,
            text_type: text_type,
            children: children,
        })
    }

    




}