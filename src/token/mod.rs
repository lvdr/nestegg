use regex::Regex;

pub fn munch_token<'a>(input: &'a str, re: &Regex) -> Option<(&'a str, &'a str)> {
    let mat = re.find(input)?;

    // If the match happens at the start of the input...
    if mat.start() == 0 {
        // ... Return the matched token and the munched input
        let matched_token = &input[..mat.end()];
        let rest_of_input = &input[mat.end()..];

        Some((matched_token, rest_of_input))
    } else {
        // ... If not, then we couldn't munch the token
        None
    }
}

pub struct TokenRule {
    name: &'static str,
    regex: Regex,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub name: &'static str,
    pub text: &'a str
}

pub fn tokenize<'a>(input: &'a str, token_rules: &Vec<TokenRule>) -> Result<Vec<Token<'a>>, &'static str> {
    let mut tokens = vec![];
    let mut partially_tokenized_input = input;

    // Trim whitespace at the start
    partially_tokenized_input = partially_tokenized_input.trim_start();

    while partially_tokenized_input.len() > 0 {

        let mut was_any_rule_applicable = false;

        // Apply every token rule and attempt to much them
        for rule in token_rules {
            if let Some((token, rest_of_input)) = munch_token(partially_tokenized_input, &rule.regex) {
                tokens.push(Token {
                    name: rule.name,
                    text: token
                });
                partially_tokenized_input = &rest_of_input[..];

                was_any_rule_applicable = true;

                break;
            }
        }

        // If we get to here in the loop without applying any rule, it means there's an unrecognizable token in
        // the input. We should just return an error
        if !was_any_rule_applicable {
            return Err("Unrecognized token in input");
        }

        // Trim whitespace at the start
        partially_tokenized_input = partially_tokenized_input.trim_start();
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod describe_munch_token {
        use super::*;

        #[test]
        fn it_deals_with_static_word_tokens() {
            let input = "Hello, world!";
            let regex = Regex::new("Hello").unwrap();

            let (token, rest_of_input) = munch_token(input, &regex).unwrap();
            assert_eq!(token, "Hello");
            assert_eq!(rest_of_input, ", world!");
        }

        #[test]
        fn it_deals_with_digits() {
            let input = "475 + 232";
            let regex = Regex::new(r"\d+").unwrap();

            let (token, rest_of_input) = munch_token(input, &regex).unwrap();
            assert_eq!(token, "475");
            assert_eq!(rest_of_input, " + 232");
        }
    }

    mod describe_tokenize {
        use super::*;

        #[test]
        fn it_can_tokenize_math_expressions() {
            let input = "678 + 232 / 21";
            let rules = vec![
                TokenRule {
                    name: "Number",
                    regex: Regex::new(r"\d+").unwrap(),
                }, TokenRule {
                    name: "Operator",
                    regex: Regex::new(r"\+|\-|/|\*").unwrap(),
                }
            ];

            let tokens = tokenize(input, &rules).unwrap();

            assert_eq!(tokens, vec![
                Token {
                    name: "Number",
                    text: "678"
                }, Token {
                    name: "Operator",
                    text: "+"
                }, Token {
                    name: "Number",
                    text: "232",
                }, Token {
                    name: "Operator",
                    text: "/",
                }, Token {
                    name: "Number",
                    text: "21",
                }
            ])
        }

    }
}
