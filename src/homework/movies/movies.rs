use chrono::{DateTime, Local};
use serde::Deserialize;
use std::cmp::{self, Ordering};

#[derive(Debug, Deserialize)]
pub struct Movie {
    movie: String,
    year: u8,
    production_budget: u64,
    domestic_gross: u64,
    foreign_gross: u64,
    worldwide_gross: u64,
    month: u8,
    profit: i64,
    profit_margin: f64,
    roi: f64,
    pct_foreign: f64,
    match_key: String,
    popularity: f64,
    release_date: DateTime<Local>,
    original_language: String,
    vote_average: f64,
    vote_count: u64,
    genre_list: Vec<String>,
    genres: String,
    action: bool,
    adventure: bool,
    animation: bool,
    comedy: bool,
    crime: bool,
    documentary: bool,
    drama: bool,
    family: bool,
    fantasy: bool,
    history: bool,
    horror: bool,
    music: bool,
    mystery: bool,
    romance: bool,
    science_fiction: bool,
    tv_movie: bool,
    thriller: bool,
    war: bool,
    western: bool,
}

impl Movie {
    fn cmp_by_attribute<T: PartialOrd>(attr1: &T, attr2: &T) -> Ordering {
        return attr1.partial_cmp(attr2).unwrap();
    }
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
    }
}

impl Eq for Movie {}

impl Ord for Movie {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year.cmp(&other.year)
    }
}

impl PartialOrd for Movie {
    fn ge(&self, other: &Self) -> bool {
        self.year >= other.year
    }

    fn gt(&self, other: &Self) -> bool {
        self.year > other.year
    }

    fn le(&self, other: &Self) -> bool {
        self.year <= other.year
    }

    fn lt(&self, other: &Self) -> bool {
        self.year < other.year
    }

    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.year.partial_cmp(&other.year)
    }
}
