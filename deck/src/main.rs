// mod games;

use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        // rand SliceRandomの拡張メソッド「.shuffleメソッド」を利用
        // Vec<T>に.shuffleメソッドが追加される
        self.cards.shuffle(&mut rng);
    }

    // usizeはOSに合わせて32bit or 64bitのサイズ幅を利用すること
    // 可変長(Vectorなど)の場合はisizeやusizeを利用する
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        // split_offで指定したindex以降がvectorから取得される
        // 全て取得する場合は0を指定
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Heres your hand: {:#?}", deck.deal(1));
}
