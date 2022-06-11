use regex::Regex;

pub fn munch_token<'a>(input: &'a str, re: Regex) -> Option<(&'a str, &'a str)> {
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
