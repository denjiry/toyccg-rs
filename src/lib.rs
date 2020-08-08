use lindera::tokenizer::{Token, Tokenizer};

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        if let Ok(parsed) = parse("日常的な推論でも、数学的に証明できるはずである。")
        {
            assert!(parsed.len() > 0);
        }
    }
}

pub fn parse<'a>(ja: &'a str) -> std::io::Result<Vec<String>> {
    let mut tokenizer = Tokenizer::new("normal", "");
    let tokens: Vec<Token<'a>> = tokenizer.tokenize(&ja);
    let mut ret: Vec<String> = Vec::new();
    for token in tokens {
        ret.push(token.text.to_string());
    }
    Ok(ret)
}
