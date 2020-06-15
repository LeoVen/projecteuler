/// If the numbers 1 to 5 are written out in words: one, two, three, four, five,
/// then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
/// If all the numbers from 1 to 1000 (one thousand) inclusive were written out
/// in words, how many letters would be used?
/// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
/// forty-two) contains 23 letters and 115 (one hundred and fifteen) contains
/// 20 letters. The use of "and" when writing out numbers is in compliance with
/// British usage.

#[derive(Debug)]
struct NumberDigits {
    pub n: usize, // Original number
    pub h: usize, // Hundreds
    pub t: usize, // Tens
    pub o: usize, // Ones
}

impl NumberDigits {
    fn new(n: usize) -> Self {
        Self {
            n,
            h: n / 100,
            t: (n / 10) % 10,
            o: n % 10,
        }
    }
}

fn spell_under_10(n: &NumberDigits) -> String {
    match n.o {
        0 => if n.n == 0 { String::from("zero") } else { String::from("") },
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        _ => unreachable!(),
    }
}

fn spell_under_20(n: &NumberDigits) -> String {
    return match n.o {
        0 => String::from("ten"),
        1 => String::from("eleven"),
        2 => String::from("twelve"),
        3 => String::from("thirteen"),
        4 => String::from("fourteen"),
        5 => String::from("fifteen"),
        6 => String::from("sixteen"),
        7 => String::from("seventeen"),
        8 => String::from("eighteen"),
        9 => String::from("nineteen"),
        _ => unreachable!(),
    }
}

fn spell_under_100(n: &NumberDigits) -> String {
    let r = match n.t {
        0 => String::from(""),
        1 => { return spell_under_20(&n); },
        2 => String::from("twenty"),
        3 => String::from("thirty"),
        4 => String::from("forty"),
        5 => String::from("fifty"),
        6 => String::from("sixty"),
        7 => String::from("seventy"),
        8 => String::from("eighty"),
        9 => String::from("ninety"),
        _ => unreachable!(),
    };
    r + &spell_under_10(&n)
}

fn spell_under_1000(n: &NumberDigits) -> String {
    let r = match n.h {
        0 => String::from(""),
        1 => String::from("onehundred"),
        2 => String::from("twohundred"),
        3 => String::from("threehundred"),
        4 => String::from("fourhundred"),
        5 => String::from("fivehundred"),
        6 => String::from("sixhundred"),
        7 => String::from("sevenhundred"),
        8 => String::from("eighthundred"),
        9 => String::from("ninehundred"),
        _ => unreachable!(),
    };
    r + &spell_and(&n) + &spell_under_100(&n)
}

fn spell_and(n: &NumberDigits) -> String {
    if (n.t != 0 || n.o != 0) && n.h != 0 {
        return String::from("and");
    }
    String::from("")
}

fn num_to_spelling(n: NumberDigits) -> String {
    if n.n > 1000 {
        unreachable!("The number must be less than 1001");
    }

    if n.n == 1000 {
        return String::from("onethousand");
    } else if n.n == 0 {
        return String::from("zero");
    }

    spell_under_1000(&n)
}

fn main() {
    println!("{}", (1..=1000)
                    .map(NumberDigits::new)
                    .map(num_to_spelling)
                    .map(|s| s.len())
                    .fold(0, |acc, x| acc + x));
}
