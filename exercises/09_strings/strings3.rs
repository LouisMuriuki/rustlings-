// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let output: String = input.trim().parse().expect("Failed to trim");
    output
}

fn compose_me(input: &str) -> String {
    let mut hello: String = input.parse().expect("Failed to trim");
    let world = " world!";
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    hello.push_str(world); // modifys the string in place and returns () i.e"
    // let hello_world = hello.push_str(world); // we get output of () which isnt the case
    hello
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
