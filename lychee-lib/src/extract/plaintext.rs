use crate::{helpers::url, types::raw_uri::RawUri};

/// Extract unparsed URL strings from plaintext
pub(crate) fn extract_plaintext(input: &str) -> Vec<RawUri> {
    url::find_links(input)
        .map(|uri| RawUri::from(uri.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_link_at_end_of_line() {
        let input = "https://www.apache.org/licenses/LICENSE-2.0\n";
        let uri = RawUri::from(input.trim_end());

        let uris: Vec<RawUri> = extract_plaintext(input);
        assert_eq!(vec![uri], uris);
    }
}