
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde_json::from_str;
use std::fs::read_to_string;

#[derive(Deserialize, Debug, Clone)]
struct Card {
    id: u32,
    name: String,
    description: String,
    cost: String,
    power: String,
    ability: String,
    rarity: String,
    status: String,
    carddefid: String,
    source: String,
    tags: String,
}

fn main() {
    let card_data = read_card_data("../cards.json").unwrap();
    let mut deck = build_deck(&card_data);
    shuffle_deck(&mut deck);
    let starting_hand = draw_cards(&mut deck, 6);
    println!("Starting Hand:");
    for card in starting_hand {
        println!(
            "Name: {}, Cost: {}, Power: {}, Ability: {}",
            card.name, card.cost, card.power, card.ability
        );
    }
}

fn read_card_data(file_path: &str) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let file_content = read_to_string(file_path)?;
    let cards: Vec<Card> = from_str(&file_content)?;
    Ok(cards)
}

fn build_deck(card_data: &[Card]) -> Vec<Card> {
    card_data.to_vec()
}

fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
}

fn draw_cards(deck: &mut Vec<Card>, number_of_cards: usize) -> Vec<Card> {
    let mut cards_drawn = Vec::new();
    for _ in 0..number_of_cards {
        if let Some(card) = deck.pop() {
            cards_drawn.push(card);
        }
    }
    cards_drawn
}
