use std::io;

#[derive(Debug)]
enum Error {
    Parse(String),
    Io(io::Error),
}

impl From<String> for Error {
    fn from(input: String) -> Error {
        Error::Parse(input)
    }
}

impl From<io::Error> for Error {
    fn from(input: io::Error) -> Error {
        Error::Io(input)
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Push(Vec<String>),
    Pop,
}

impl Command {
    pub fn is_push(&self) -> bool {
        if let Command::Push(_) = self {
            true
        } else {
            false
        }
    }
}

/// Adapt your parsing code to have a parsing function
/// with this signature. Inside, make heavy use of the
/// try operator (`let res = may_fail()?`).
fn parse<S: AsRef<str>>(input: S) -> Result<Command, Error> {
    let input_str = input.as_ref();
    println!("trying input {:?}", input_str);

    // uncomment this to demo shrinking, then look at the
    // proptest regressions file.
    /*
    if input_str.len() >= 7 &&
        input_str.contains('!') &&
        input_str.starts_with("PUSH") {
        panic!("OH NO");
    }
    */

    if input_str.starts_with("POP") {
        Ok(Command::Pop)
    } else if input_str.starts_with("PUSH") {
        let items = input_str[4..]
            .split(',')
            .map(|i| String::from(i))
            .collect();
        Ok(Command::Push(items))
    } else {
        Err(Error::Parse("could not parse input".into()))
    }
}

#[cfg(test)]
mod tests {
    use proptest::*;

    use super::*;

    #[test]
    fn parses_pop() {
        assert_eq!(
            parse("POP\n").expect("should not return an Err"),
            Command::Pop,
            "We expect that our system can parse POP",
        );
    }

    proptest! {
        #[test]
        fn doesnt_crash(ref input in "(PUSH){0,3}(POP){0,3}.*") {
            // proptest will use panic::catch_unwind to safely
            // catch the panic if it happens
            let _ = parse(input);
        }

        #[test]
        fn parses_push(ref input in "PUSH(\\s)*(\\w*,)*") {
            let result = parse(input).expect("should not return an Error");
            assert!(result.is_push(), "should properly parse pushes");
        }

        #[test]
        fn returns_err_with_bad_command(ref input in "(PUSH){0}(POP){0}.*") {
            parse(input).expect_err("should return an Error");
        }
    }
}
