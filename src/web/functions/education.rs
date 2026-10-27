use futures::future::try_join_all;
use thirtyfour::{
    By, WebDriver, WebElement, error::WebDriverError, extensions::query::ElementQueryable,
};

use crate::{config::CONFIG, database::models::Course};

const BASE_URL: &str = "https://linkedin.com/school/";

pub enum CourseError {
    WebDriverError(WebDriverError),
    UnknownCourse(Vec<String>),
    NoUniEducation,
}

async fn get_course_name(headline_element: &WebElement) -> Result<String, CourseError> {
    let headline = headline_element
        .text()
        .await
        .map_err(|x| CourseError::WebDriverError(x))?;
    let course_str = headline.split(',').nth(1).unwrap().trim().to_lowercase();

    Ok(course_str)
}

fn get_course_id(course_name: &str, courses: &Vec<Course>) -> Option<i32> {
    courses
        .iter()
        .find(|x| course_name.contains(&x.name))
        .map(|x| x.id)
}

pub async fn get_education_courses(
    driver: &WebDriver,
    courses: &Vec<Course>,
) -> Result<Vec<i32>, CourseError> {
    // "Kurso lygis, Kursas, ..."
    let course_headlines = &driver
        .query(By::Css(format!(
            "a[href=\"{}{}\"] > div p:nth-of-type(2)",
            BASE_URL, &CONFIG.fetch.university_id
        )))
        .desc("Absolvento universiteto kursas")
        .any()
        .await
        .map_err(|x| CourseError::WebDriverError(x))?;

    // Gali buti, kad pateike, kad mokesi universitete, bet neparaso kurso, kuri lanke
    if course_headlines.len() < 1 {
        return Err(CourseError::NoUniEducation);
    }

    // Pasiimti kurso pavadinimus ir isaugoti, kad geresnius error rodytu
    let course_names = try_join_all(
        course_headlines
            .iter()
            .map(|headline_element| get_course_name(headline_element)),
    )
    .await?
    .iter()
    .cloned()
    .collect::<Vec<String>>();

    // Kurso IDs, kurie bus ideti i duomenu baze susieje su absolventu
    let course_ids = course_names
        .iter()
        .filter_map(|course_name| get_course_id(course_name, courses))
        .collect::<Vec<i32>>();

    if course_ids.len() < 1 {
        return Err(CourseError::UnknownCourse(course_names));
    }

    Ok(course_ids)
}
