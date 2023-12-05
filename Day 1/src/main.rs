use std::fs;

fn main() {

    let input_file = fs::read_to_string("src/input.txt");
    if input_file.is_err() {
        println!("Failed to open text file.");
        return;
    } 

    let input_file = input_file.unwrap();

    println!("{:?}", input_file);

    let mut sum: u32 = 0;

    for line in input_file.lines() {
        let number = parse_number_from_line(line);
        sum += number as u32;
    }

    println!("Your sum is: {:?}", sum);
}

fn parse_number_from_line(line: &str) -> u8 {
    let mut first_character: Option<char> = None;
    let mut last_character: Option<char> = None;

    for character in line.chars() {
        match character {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if first_character.is_none() {
                    first_character = Some(character);
                }
                last_character = Some(character);
            },
            _ => {}
        }
    }

    if first_character.is_none() || last_character.is_none() {
        println!("Bad input data, line without number");
        return 0;
    }
    let first_character = first_character.unwrap();
    let last_character  = last_character.unwrap();

    let mut string_representation: String = "".to_string();
    string_representation += &first_character.to_string();
    string_representation += &last_character.to_string();

    return string_representation.parse().unwrap();
}

#[test]
fn test_find_number() {
    assert_eq!(parse_number_from_line("treb7uchet"), 77);
    assert_eq!(parse_number_from_line("a1b2c3d4e5f"), 15);
    assert_eq!(parse_number_from_line("pqr3stu8vwx"), 38);
    assert_eq!(parse_number_from_line("1abc2"), 12);
}
