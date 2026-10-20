const BASE_URL: &str = "https://linkedin.com/";

pub fn get_profile_url(id: &str) -> String {
    format!("{}/in/{}/", BASE_URL, id)
}
