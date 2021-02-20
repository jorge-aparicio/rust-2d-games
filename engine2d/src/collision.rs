use crate::objects::{MovingRect, Rect};

pub struct Contact(ContactID, ContactID);

pub enum ContactID {
    Obstacle(usize),
    Player,
}

fn gather_contacts(player: Rect, obstacles: &[MovingRect]) -> Vec<Contact> {
    let mut contacts = Vec::new();
    for (i, obstacle) in obstacles.iter().enumerate() {
        if (player.pos.x + player.size.x >= obstacle.pos.x
            || player.pos.x <= obstacle.pos.x + obstacle.size.x)
            && (player.pos.y + player.size.y >= obstacle.pos.y
                || player.pos.y <= obstacle.pos.y + obstacle.size.y)
        {
            contacts.push(Contact(ContactID::Player, ContactID::Obstacle(i)));
        }
    }
    contacts
}
