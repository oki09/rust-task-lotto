use std::env;

use rand::prelude::SliceRandom;

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let mut rng = &mut rand::thread_rng();
        let samples: Vec<usize> = (1..=from).collect();
        Lotto {
            take: take,
            from: from,
            numbers: samples.choose_multiple(&mut rng, take).cloned().collect()
        }
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    // Tip: Use the format macro
    format!("{} of {}: {:?}", lotto.take, lotto.from, lotto.numbers)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let slice = &args[1..];
    let mut take: usize = 0;
    for (idx, elem) in slice.iter().enumerate() {
        if (idx + 1) % 2 == 0 {
            let from: usize = elem.parse().unwrap();
            let lotto = Lotto::new(take, from);
            println!("{}", format_lotto_results(&lotto));
        } else {
            take = elem.parse().unwrap();
        }
    }
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.numbers;

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.numbers;
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
