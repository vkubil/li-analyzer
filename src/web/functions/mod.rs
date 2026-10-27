pub mod details;
pub mod education;
pub mod experience;

pub fn get_profile_details_page(profile_url: &str, page: &str) -> String {
    format!("{}/details/{}/", profile_url, page)
}
