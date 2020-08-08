use lindera::tokenizer::Tokenizer;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_tokenize() {
        let test_str = "日常的な推論でも、数学的に証明できるはずである。";
        if let Ok(tokenized) = tokenize(test_str) {
            assert!(tokenized.len() > 0);
        }
    }
    #[test]
    fn test_build_chart() {
        let test_str = "日常的な推論でも、数学的に証明できるはずである。";
        if let Ok(tokenized) = tokenize(test_str) {
            let tokenized_length = tokenized.len();
            let chart = build_chart(tokenized);
            assert_eq!(chart.len(), tokenized_length);
            // assert_eq!(chart[0][0], "a".to_string());
        }
    }
}

pub fn build_chart(tokens: Vec<String>) -> Vec<Vec<String>> {
    let chart_size = tokens.len();
    let mut chart = vec![vec![String::new(); chart_size]; chart_size];
    for (n, chart_row) in chart.iter_mut().enumerate().take(chart_size) {
        for cell in chart_row.iter_mut().take(chart_size).skip(n) {
            *cell = "a".to_string();
        }
    }
    chart
}

pub fn tokenize(ja: &str) -> std::io::Result<Vec<String>> {
    let mut tokenizer = Tokenizer::new("normal", "");
    let tokens = tokenizer.tokenize(&ja);
    let mut ret: Vec<String> = Vec::new();
    for token in tokens {
        ret.push(token.text.to_string());
    }
    Ok(ret)
}
