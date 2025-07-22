fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.

    if let Some(start_index) = input.find(|c| c != ' ') {
        let end_index = input.rfind(|c| c != ' ').unwrap();

        &input[start_index..end_index + 1]
    } else {
        ""
    }
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    const SECOND_PART: &str = " world!";

    let mut combined = String::with_capacity(input.len() + SECOND_PART.len());

    combined.push_str(input);
    combined.push_str(SECOND_PART);

    combined
}

fn replace_me(input: &str, target: &str, replacement: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let occurrences: Vec<(usize, _)> = input.match_indices(target).collect();

    let replacement_enlargement = (replacement.len() as isize) - (target.len() as isize);
    let input_enlargement = replacement_enlargement * (occurrences.len() as isize);

    let mut replaced = String::with_capacity(((input.len() as isize) + input_enlargement) as usize);

    let mut after_occurrence: usize = 0;

    for (start_of_occurrence, _) in occurrences {
        // Push before occurrence
        replaced.push_str(&input[after_occurrence..start_of_occurrence]);

        replaced.push_str(replacement);
        after_occurrence = start_of_occurrence + target.len();
    }

    // Push after last occurrence
    replaced.push_str(&input[after_occurrence..]);

    replaced
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");

        // These two were added
        assert_eq!(trim_me("   "), "");
        assert_eq!(trim_me(""), "");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool", "cars", "balloons"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars", "cars", "balloons"),
            "I love to look at balloons",
        );
        assert_eq!(
            replace_me(
                "These crocodiles are cool. I love to look at crocodiles.",
                "crocodiles",
                "cats"
            ),
            "These cats are cool. I love to look at cats.",
        );
        assert_eq!(
            replace_me("crocodiles are nice", "crocodiles", "cats"),
            "cats are nice",
        );
    }
}
