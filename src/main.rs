#[derive(Debug)]
struct Letter {
    index: usize,
    alphabet: char,
    alphabetic_index: usize,
}

#[derive(Debug)]
struct MessageMatrix {
    message: Vec<Vec<Letter>>,
}

type Keyword = Vec<Letter>;

impl Letter {
    fn construct(index: usize, letter: char, a_index: usize) -> Self {
        Letter {
            index,
            alphabet: letter,
            alphabetic_index: a_index,
        }
    }

    fn shift(keyword: String) -> Keyword {
        if !keyword.is_ascii() {
            panic!("String contains non-ASCII characters")
        }
        let mut letters: Keyword = keyword
            .to_uppercase()
            .chars()
            .enumerate()
            .map(|(index, letter)| Letter::construct(index, letter, 0))
            .collect();

        letters.sort_by(|x, y| {
            x.alphabet
                .to_ascii_lowercase()
                .cmp(&y.alphabet.to_ascii_lowercase())
        });

        letters
            .iter_mut()
            .enumerate()
            .for_each(|(index, x)| x.alphabetic_index = index);

        letters
    }
}

impl MessageMatrix {
    fn encrypt(keyword: Keyword, mut message: String) -> String {
        if !message.is_ascii() {
            panic!("String contains non-ASCII characters")
        }
        message = message.to_uppercase().replace(' ', "");
        let step = keyword.len();
        let loop_time = ((message.chars().count() / step) as f32).ceil() as usize;
        let mut message_matrix = MessageMatrix {
            message: Vec::new(),
        };

        for _ in 0..loop_time {
            let mut row = String::from(&message);
            if row.len() > step {
                row = String::from(message.get(0..step).unwrap());
            }
            let row_in_matrix: Vec<Letter> = row
                .chars()
                .enumerate()
                .map(|(index, letter)| Letter::construct(index, letter, 0))
                .collect();
            message_matrix.message.push(row_in_matrix);
            message = String::from(message.split_at(6).1);
        }

        if message.chars().count() < step {
            for _ in 0..step - message.chars().count() {
                message.push('X');
            }
            message_matrix.message.push(
                message
                    .chars()
                    .enumerate()
                    .map(|(index, letter)| Letter::construct(index, letter, 0))
                    .collect(),
            )
        }
        println!("MATRIX :");
        message_matrix
            .message
            .iter()
            .enumerate()
            .for_each(|(_, row)| {
                print!("\n");
                row.iter().for_each(|c| {
                    print!("     {}", c.alphabet);
                })
            });

        println!();
        // Get the actual encoded message
        let mut indexes_to_fetch = Vec::new();
        let mut cipher_chars = Vec::<char>::new();
        keyword.iter().for_each(|x| indexes_to_fetch.push(x.index));

        indexes_to_fetch.iter().for_each(|index| {
            message_matrix
                .message
                .iter()
                .for_each(|row| cipher_chars.push(row.get(*index).unwrap().alphabet))
        });
        println!("\nCipher text:");
        // cipher_chars.iter().for_each(|c| print!("{}", c));
        let cipher = cipher_chars.iter().map(|c| c).collect::<String>();
        cipher
    }

    // TODO: Cipher decrypt
}

fn main() {
    print!(
        "**********************************************\n     Columnar Transposition algorithm\n**********************************************\n"
    );
    let keyword: String = String::from("tomato");
    let plain_text: String = String::from("My favorite fruit is watermelon");
    println!("**Keyword**: {}\n**Plain text**: {}\n", keyword, plain_text);
    let letters = Letter::shift(keyword);
    let cipher = MessageMatrix::encrypt(letters, plain_text);
    println!("{}", cipher);
}
