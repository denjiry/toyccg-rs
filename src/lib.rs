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
    fn test_buildChart() {
        let test_str = "日常的な推論でも、数学的に証明できるはずである。";
        let Ok(tokenized) = tokenize(test_str);
        let chart = build_chart(tokenized);
        assert_eq!(chart.len(), tokenized.len());
    }
}

pub fn build_chart(tokens: Vec<String>) -> Vec<Vec<String>> {
    let chart_size = tokens.len();
    let mut chart = vec![vec![String::new(); chart_size]; chart_size];
    for n in 0..chart_size {
        for m in n..chart_size {
            chart[n][m] = "a".to_string();
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
