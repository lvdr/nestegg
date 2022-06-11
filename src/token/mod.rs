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

pub struct Token<'a> {
    name: &'static str,
    text: &'a str
}

pub fn tokenize<'a>(input: &'a str, token_rules: &Vec<TokenRule>) -> Vec<Token<'a>> {
    let mut tokens = vec![];
    let mut partially_tokenized_input = input;

    while partially_tokenized_input.len() > 0 {
        for rule in token_rules {
            if let Some((token, rest_of_input)) = munch_token(partially_tokenized_input, &rule.regex) {
                tokens.push(Token {
                    name: rule.name,
                    text: token
                });
                partially_tokenized_input = &rest_of_input[..];

                break;
            }
        }
    }

    return tokens;
}
