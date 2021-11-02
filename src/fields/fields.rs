use std::io::stdin;
use rand::Rng;

static RANKS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

#[derive(PartialEq)]
enum FieldColor {
    Dark,
    Light
}

pub fn fields_game() {
    loop {
        let field = generate_random_field();  
        println!("{}: light or dark?", field);
        let raw_input = get_user_input();
        let input = match get_field_color_from_input(raw_input.clone()) {
            Some(s) => s,
            None => {
                println!("unrecognized option: {}", raw_input);
                continue;
            }
        };

        if calculate_field_color(field) == input {
            println!("Correct! ✅")
        }
        else {
            println!("Incorrect! ❌")
        }
    }
}

fn calculate_field_color(field: String) -> FieldColor {
    let rank = &field[..1];
    let field_number: i32 = field[1..].parse().unwrap();

    let index = RANKS.iter().position(|&r| r == rank).unwrap() as i32;
    
    if is_even(index) && is_even(field_number) || !is_even(index) && !is_even(field_number) {
        return FieldColor::Light;
    } 

    FieldColor::Dark
}


fn generate_random_field() -> String {

    let mut rng = rand::thread_rng();
    let rank = rng.gen_range(1..RANKS.len());
    let field = rng.gen_range(1..RANKS.len());

    String::from(format!("{}{}", RANKS[rank], field))
}

fn get_user_input() -> String {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
    
    remove_whitespace(&mut input_string).to_lowercase()
}

fn get_field_color_from_input(input: String) -> Option<FieldColor> {
    let allowed_light_answers = 
        ["l".to_string(),  "w".to_string(), 
         "light".to_string(), "white".to_string()
        ];
         
    let allowed_dark_answers = [
        "b".to_string(), "d".to_string(),
        "dark".to_string(), "black".to_string()
    ];

    if allowed_light_answers.contains(&input) {
        return Some(FieldColor::Light);
    }

    if allowed_dark_answers.contains(&input) {
        return Some(FieldColor::Dark);
    }

    None
}

fn remove_whitespace(s: &mut String) -> String {
    s.retain(|c| !c.is_whitespace());
    s.to_owned()
}

fn is_even(input: i32) -> bool {
    input % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calc_e4_and_d4() {
        let e4 = calculate_field_color("e4".to_string());
        let d4 = calculate_field_color("d4".to_string());
        assert!(matches!(e4, FieldColor::Light));
        assert!(matches!(d4, FieldColor::Dark));
    }

    #[test]
    fn calc_f4_and_c6() {
        let f4 = calculate_field_color("f4".to_string());
        let c6 = calculate_field_color("c6".to_string());
        assert!(matches!(c6, FieldColor::Light));
        assert!(matches!(f4, FieldColor::Dark));
    }

    #[test]
    fn calc_a8_h8_a1_h1() {
        let a8 = calculate_field_color("a8".to_string());
        let h8 = calculate_field_color("h8".to_string());
        let a1 = calculate_field_color("a1".to_string());
        let h1 = calculate_field_color("h1".to_string());
        assert!(matches!(a8, FieldColor::Light));
        assert!(matches!(h8, FieldColor::Dark));
        assert!(matches!(a1, FieldColor::Dark));
        assert!(matches!(h1, FieldColor::Light));
    }

}