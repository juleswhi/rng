use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        let num: u8 = rng.gen_range(0..101);
        println!("Your number is: \x1b[1m\x1b[38;5;3m{}\x1b[22m", num);
        return;
    }

    match args[1].as_str() {
        "card" => {
            let rank = match rng.gen_range(0..13) {
                0 => Rank::Ace,
                1 => Rank::Two,
                2 => Rank::Three,
                3 => Rank::Four,
                4 => Rank::Five,
                5 => Rank::Six,
                6 => Rank::Seven,
                7 => Rank::Eight,
                8 => Rank::Nine,
                9 => Rank::Ten,
                10 => Rank::Jack,
                11 => Rank::Queen,
                _ => Rank::King,
            };

            let suit = match rng.gen_range(0..4) {
                0 => Suit::Hearts,
                1 => Suit::Diamonds,
                2 => Suit::Clubs,
                _ => Suit::Spades,
            };
            println!("Your card is: \x1b[1m\x1b[38;5;3m{} of {}\x1b[22m", rank.to_string(), suit.to_string());
        }
        _ => {}
    }
}

#[derive(Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    fn to_string(&self) -> String {
        let suit_str = match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };

        suit_str.to_string()
    }
}

#[derive(Debug)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    fn to_string(&self) -> String {
        let rank_str = match self {
            Rank::Ace => "Ace",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
        };
        rank_str.to_string()
    }
}
