use rand::{rng,  seq::SliceRandom};
// use rand::rng;
// use rand::seq::SliceRandom;

#[derive(Debug)]
struct Deck {
  cards: Vec<String>,
}

impl Deck {
  fn new() -> Self {
    let suits = vec!["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    let mut cards = vec![];

    for suit in &suits {
      for value in &values {
        let card = format!("{} of {}", value, suit);
    
        cards.push(card);
      }
    }

    Deck { cards }
  }

  fn shuffle(&mut self) {
    let mut random = rng();

    self.cards.shuffle(&mut random);
  }

  fn deal(&mut self, num_cards: usize) -> Vec<String> {
    self.cards.split_off(self.cards.len() - num_cards)
  }
}

fn main() {
  let mut deck = Deck::new();

  // deck.shuffle();

  let cards = deck.deal(3);
  
  println!("Heres your hand {:#?}: ", cards);
  println!("Heres your deck {:#?}: ", deck);
}
