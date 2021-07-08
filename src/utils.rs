use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn handle_google_search(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let google_search_url = format!("https://google.com/search?q={}", encoded_query);

    google_search_url
}

pub fn handle_github_search(query: &str) -> String {
    if query == "gh" {
        return String::from("https://github.com");
    }

    let gh_page = query.split(" ").collect::<Vec<&str>>()[1];
    let github_search_url = format!("https://github.com/{}", gh_page);

    github_search_url
}

pub fn handle_twitch_search(query: &str) -> String {
    if query == "tw" {
        return String::from("https://twitch.tv");
    }

    let tw_page = query.split(" ").collect::<Vec<&str>>()[1];
    let twitch_search_url = format!("https://twitch.tv/{}", tw_page);

    twitch_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_search() {
        let spaced_word = "learn rust";
        let single_word = "hello";

        assert_eq!(
            "https://google.com/search?q=learn%20rust",
            handle_google_search(spaced_word)
        );
        assert_eq!(
            "https://google.com/search?q=hello",
            handle_google_search(single_word)
        );
    }

    #[test]
    fn test_github_search() {
        let github_home = "gh";
        let github_repo = "gh test_user/test_repo";

        assert_eq!("https://github.com", handle_github_search(github_home));
        assert_eq!(
            "https://github.com/test_user/test_repo",
            handle_github_search(github_repo)
        );
    }

    #[test]
    fn test_twitch_search() {
        let twitch_home = "tw";
        let twitch_streamer = "tw CoolStreamer";

        assert_eq!("https://twitch.tv", handle_twitch_search(twitch_home));
        assert_eq!(
            "https://twitch.tv/CoolStreamer",
            handle_twitch_search(twitch_streamer)
        );
    }
}
