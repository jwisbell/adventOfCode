use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

//today took heavy inspiration from a reddit post which showed how to derive Ord, Partial ord for the hands and greatly simplify things
//once the cards are properly parsed

pub fn main(){
    //let filename = "./inputs/day7_test.txt";
    let filename = "./inputs/day7.txt";
    let (inputs, inputs_p2) = process_input(filename);
    
    //println!("{:?}", inputs);
    
    part1(inputs.clone());
    part2(inputs_p2.clone());

    
}

fn part1(data:Vec<(Vec<CardRank>, u32)>){
    //multiply each hand's bid and rank
    println!("Part 1");
    let mut sum = 0;
    
    //first compute all the ranks and sort
    let mut sorted_hands = data.clone();
    sorted_hands.sort_by(|x,y| compare_hands(&x.0, &y.0, 1));

    //now compute the puzzle value of rank*bid
    for i in 0..sorted_hands.len(){
        sum += (i+1) as u32  * sorted_hands[i].1;
    }
    println!("The sum is {}", sum);
}



fn part2(data:Vec<(Vec<CardRank>, u32)>){
    //now there are jokers...
    println!("Part 2");

    //multiply each hand's bid and rank
    let mut sum = 0;
    
    //first compute all the ranks and sort
    let mut sorted_hands = data.clone();
    sorted_hands.sort_by(|x,y| compare_hands(&x.0, &y.0, 2));

    //now compute the puzzle value of rank*bid
    for i in 0..sorted_hands.len(){
        sum += (i+1) as u32  * sorted_hands[i].1;
    }
    println!("The sum is {}", sum);
}


#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CardRank {
    Wild,
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
    Ace,
}

impl CardRank {
    fn new(r: char) -> CardRank {
        match r {
            '2' => CardRank::Two,
            '3' => CardRank::Three,
            '4' => CardRank::Four,
            '5' => CardRank::Five,
            '6' => CardRank::Six,
            '7' => CardRank::Seven,
            '8' => CardRank::Eight,
            '9' => CardRank::Nine,
            'T' => CardRank::Ten,
            'J' => CardRank::Jack,
            'Q' => CardRank::Queen,
            'K' => CardRank::King,
            'A' => CardRank::Ace,
            _ => panic!("unexpected char"),
        }
    }

    fn new_part2(r: char) -> CardRank {
        match r {
            '2' => CardRank::Two,
            '3' => CardRank::Three,
            '4' => CardRank::Four,
            '5' => CardRank::Five,
            '6' => CardRank::Six,
            '7' => CardRank::Seven,
            '8' => CardRank::Eight,
            '9' => CardRank::Nine,
            'T' => CardRank::Ten,
            'J' => CardRank::Wild,
            'Q' => CardRank::Queen,
            'K' => CardRank::King,
            'A' => CardRank::Ace,
            _ => panic!("unexpected char"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Hand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn rank_hand(hand:&[CardRank])->Hand{
    //build a histogram of card counts
    let mut histo: HashMap<CardRank, u32> = HashMap::new();
    for card in hand.iter() {
        histo.entry(*card).and_modify(|e| *e += 1).or_insert(1);
    }

    //now process
    let mut result: Hand = Hand::HighCard; //default to high card

    if histo.values().len() == 1 { //if there is only one type of card
        result = Hand::FiveOfAKind;
    } else if histo.values().len() == 2 { //if there are two types of card
        let mut counts: Vec<u32> = histo.values().cloned().collect();
        counts.sort();
        if counts == [1, 4] {
            result = Hand::FourOfAKind;
        } else if counts == [2, 3] {
            result = Hand::FullHouse;
        }
    } else if histo.values().len() == 3 { //there are three card types
        let mut counts: Vec<u32> = histo.values().cloned().collect();
        counts.sort();
        if counts == [1, 1, 3] {
            result = Hand::ThreeOfAKind;
        } else if counts == [1, 2, 2] {
            result = Hand::TwoPairs;
        }
    } else if histo.values().len() == 4 { //there are four card types
        result = Hand::OnePair;
    } 

    result
}


fn rank_hand_p2(hand: &[CardRank])->Hand{
    //build the histogram
    let mut histo: HashMap<CardRank, u32> = HashMap::new();
    for card in hand.iter() {
        histo.entry(*card).and_modify(|e| *e += 1).or_insert(1);
    }
    
    
    let mut result: Hand = Hand::HighCard;
    if histo.values().len() == 1 { //still just 5 of a kind
        result = Hand::FiveOfAKind;
    } else if histo.values().len() == 2 { //there are two types of cards
        let mut counts: Vec<u32> = histo.values().cloned().collect();
        counts.sort();
        if counts == [1, 4] { //four of one, 1 of another
            if !histo.contains_key(&CardRank::Wild) { //check if one of them is wild -- if not, do same as before
                result = Hand::FourOfAKind;
            } else {
                // either 4wild+1other or 4other+1 wild, but this is equivalent
                result = Hand::FiveOfAKind;
            }
        } else if counts == [2, 3] {
            if !histo.contains_key(&CardRank::Wild) { //if no wilds, proceed as bnefore
                result = Hand::FullHouse;
            } else {
                // either 3wild+2other or 3other+2wild, but this is equivalent
                result = Hand::FiveOfAKind;
            }
        }
    } else if histo.values().len() == 3 { //three types of cards
        let mut counts: Vec<u32> = histo.values().cloned().collect();
        counts.sort();
        if counts == [1, 1, 3] {
            if !histo.contains_key(&CardRank::Wild) { //if no wilds, 3 of a kind
                result = Hand::ThreeOfAKind;
            } else {
                // up to 4 of a kind 
                result = Hand::FourOfAKind;
            }
        } else if counts == [1, 2, 2] {
            if !histo.contains_key(&CardRank::Wild) { //once again, if no wilds, proceed as before
                result = Hand::TwoPairs;
            } else if histo[&CardRank::Wild] == 1 {//one wild
                // 2 other + 2 other + 1 wild
                result = Hand::FullHouse;
            } else { //two wilds
                // 2 other + 2 wilds + 1 other
                result = Hand::FourOfAKind;
            }
        }
    } else if histo.values().len() == 4 { //four types of cards....
        if !histo.contains_key(&CardRank::Wild) { //if no wild, one card must be doubled
            result = Hand::OnePair;
        } else {
            // either 2 other + 1 wild + 1 other + 1 other
            // or 2 wilds + 1 other + 1 other + 1 other
            // use the wilds to make 3 of a kind
            result = Hand::ThreeOfAKind;
        }
    } else if !histo.contains_key(&CardRank::Wild) {//if no wild, base case
        result = Hand::HighCard;
    } else { //there are 5 different cards, but one is wild
        result = Hand::OnePair;
    }
    result
}

//function to use in sort_by --- inspired by the reddit post
fn compare_hands(first: &[CardRank], second: &[CardRank], part:usize) -> Ordering {
    let mut first_rank: Hand = rank_hand(first);
    let mut second_rank: Hand = rank_hand(second);

    if part==2 {
        first_rank = rank_hand_p2(first);
        second_rank = rank_hand_p2(second);
    }
    
    //cmp stands for compare...
    match first_rank.cmp(&second_rank) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => first.cmp(second),
    }
}

fn process_input(filename:&str) -> (Vec<(Vec<CardRank>, u32)>,Vec<(Vec<CardRank>, u32)>){
    //this part is the same for all files...
    println!("Processing {}",filename);
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    
    
    let mut hands_with_bids: Vec<(Vec<CardRank>, u32)> = vec![];
    let mut hands_with_bids_p2: Vec<(Vec<CardRank>, u32)> = vec![];
    //let mut in_pattern = false;

    for line in lines{
        if let Some((hand_str, bid_str)) = line.split_once(' ') {
            println!("{} {}", hand_str, bid_str);
            let hand: Vec<CardRank> = hand_str.chars().map(CardRank::new).collect();
            let hand_p2: Vec<CardRank> = hand_str.chars().map(CardRank::new_part2).collect();
            let bid: u32 = bid_str.trim().parse::<u32>().unwrap();
            hands_with_bids.push((hand, bid));
            hands_with_bids_p2.push((hand_p2, bid));
        }
    }

    (hands_with_bids, hands_with_bids_p2)

}
