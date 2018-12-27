fn can_react(c1: &char, c2: &char) -> bool {
    (c1.is_lowercase() && c1.to_ascii_uppercase() == *c2)
        || (c1.is_uppercase() && c1.to_ascii_lowercase() == *c2)
}

fn polymer_reaction(polymer: &mut String) -> bool {
    let polymer_length = polymer.len();
    if polymer_length <= 1 {
        return false;
    }
    let mut has_reacted: bool = false;
    let mut opt_previous_char: Option<char> = None;
    let mut reaction_index: usize = 0;
    for (char_index, char_value) in polymer.char_indices() {
        if opt_previous_char.is_some() {
            let previous_char = opt_previous_char.unwrap();
            if can_react(&char_value, &previous_char) {
                has_reacted = true;
                reaction_index = char_index - 1;
                break;
            }
        }
        opt_previous_char.replace(char_value);
    }
    let mut new_polymer: String = String::new();
    if has_reacted {
        for (char_index, char_value) in polymer.char_indices() {
            if char_index != reaction_index && char_index != (reaction_index + 1) {
                new_polymer.push(char_value)
            }
        }
        polymer.clear();
        polymer.clone_from(&new_polymer);
        return true;
    }
    false
}

fn process_polymer_reactions(polymer: &str) -> String {
    let mut new_polymer: String = String::new();
    new_polymer.push_str(polymer);
    let mut has_reacted: bool = true;
    while has_reacted {
        has_reacted = polymer_reaction(&mut new_polymer);
    }
    new_polymer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_polymers_without_reactions() {
        assert_eq!(process_polymer_reactions(""), "");
        assert_eq!(process_polymer_reactions("a"), "a");
    }

    #[test]
    fn test_reduce_to_empty_polymer() {
        assert_eq!(process_polymer_reactions("abcdDCBA"), "");
    }

    #[test]
    fn test_reduce_to_partial_polymer() {
        assert_eq!(process_polymer_reactions("cBaAbd"), "cd");
    }

    #[test]
    fn test_provided_example() {
        let input = "dabAcCaCBAcCcaDA";
        let expected_output = "dabCBAcaDA";
        assert_eq!(process_polymer_reactions(input), expected_output);
    }

    #[test]
    fn test_many_reactions() {
        let input = "abcdefghijklmnopqrstuvwxyzZYXWVUTSRQPONMLKJIHGFEDCBAabcdefghijklmnopqrstuvwxyzZYXWVUTSRQPONMLKJIHGFEDCBA";
        let expected_output = "";
        assert_eq!(process_polymer_reactions(input), expected_output);
    }
}
