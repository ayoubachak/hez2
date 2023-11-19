use bevy::math::Vec2;

use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub enum CardType {
    Flous,
    Syouf,
    Gro3,
    Jbaben,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub kind: CardType,
    pub number: i32,
    pub width : f32,
    pub height : f32,
    pub coords: (Vec2, Vec2),
    pub image_path: String,
}

impl Card {
    pub fn default_coords() -> (Vec2, Vec2) {
        (Vec2::new(0.0, 0.0), Vec2::new(100.0, 160.0))
    }

    pub fn set_coords(&mut self, coords: (Vec2, Vec2)) {
        self.coords = coords;
    }

    pub fn default_image_path(&self) -> String {
        match self.kind {
            CardType::Flous => format!("cards/f{}.jpg", self.number),
            CardType::Syouf => format!("cards/s{}.jpg", self.number),
            CardType::Gro3 => format!("cards/g{}.jpg", self.number),
            CardType::Jbaben => format!("cards/j{}.jpg", self.number),
        }
    }

    pub fn set_image_path(&mut self, image_path: &str) {
        self.image_path = image_path.to_string();
    }
}

pub struct CardDistributor {
    cards: Vec<Card>,
}

impl CardDistributor {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        let width : f32 = 100.0;
        let height: f32 = 160.0;
        for card_type in &[CardType::Flous, CardType::Syouf, CardType::Gro3, CardType::Jbaben] {
            for number in &[1, 2, 3, 4, 5, 6, 7, 10, 11, 12] {
                let mut card = Card {
                    id: cards.len() + 1,
                    kind: *card_type,
                    number: *number,
                    width : width,
                    height : height,
                    coords: Card::default_coords(),
                    image_path: String::new(),
                };

                card.set_image_path(&card.default_image_path());
                card.set_coords((Vec2::new(0.0, 0.0), Vec2::new(100.0, 160.0))); // these are the default coords
                cards.push(card);
            }
        }

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn get_cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

}
