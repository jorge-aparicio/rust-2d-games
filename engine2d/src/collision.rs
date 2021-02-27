use crate::objects::MovingRect;

pub struct Contact(ContactID, ContactID);

impl Contact {
    pub fn get_ids(&self) -> (ContactID, ContactID) {
        (self.0, self.1)
    }
}

#[derive(Copy, Clone)]
pub enum ContactID {
    Obstacle,
    Player,
}

pub fn gather_contacts(player: &MovingRect, obstacles: &[MovingRect]) -> Vec<Contact> {
    let mut contacts = Vec::new();
    for obstacle in obstacles.iter() {
        if player.pos.x <= obstacle.pos.x + obstacle.size.x &&
            obstacle.pos.x <= player.pos.x + player.size.x  &&
            player.pos.y <= obstacle.pos.y + obstacle.size.y  &&
            obstacle.pos.y <= player.pos.y + player.size.y
        {
            contacts.push(Contact(ContactID::Player, ContactID::Obstacle));
        }
    }
    contacts
}
