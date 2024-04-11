// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // Make this an if let statement whose value is "Some" type
        let word = match optional_target {
            Some(word) => assert_eq!(word, target),
            _ => ()
        };
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        loop {
            match optional_integers.pop().unwrap() {
                Some(integer) => {
                    println!("{}", &integer);
                    assert_eq!(integer, cursor);
                    cursor -= 1;
                },
                None => break,
            }
        }


        assert_eq!(cursor, 0);
    }
}
