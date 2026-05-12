use reqwest;
use tokio;

fn read_urls_from_list(f: &str) -> Result<Vec<String>, std::io::Error> {
    let contents = std::fs::read_to_string(f)?;
    let re = regex::Regex::new(r"\s*,\s*").unwrap();
    let urls: Vec<String> = re.split(&contents).map(|s| s.trim().to_string()).collect();
    Ok(urls)
}

pub async fn get_page(url: &str) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(url).await?.text().await?)
}

// pub async fn get_pages(list: Vec<String>) -> Vec<Result<String>, reqwest::Error> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_urls_from_list() {
        let path = "test.csv";
        std::fs::write(path, "https://apple.com, https://google.com").unwrap();
        let urls = read_urls_from_list(path);
        assert_eq!(
            urls.unwrap(),
            vec!["https://apple.com", "https://google.com"]
        );
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_read_urls_with_newlines() {
        let path = "test.csv";
        std::fs::write(path, "https://apple.com   ,\n https://google.com").unwrap();
        let urls = read_urls_from_list(path);
        assert_eq!(
            urls.unwrap(),
            vec!["https://apple.com", "https://google.com"]
        );
        std::fs::remove_file(path).unwrap();
    }
}
