use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserProfile {
    pub usrid: i32,
    pub usrpublic_id: String,
    pub usrcreated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TodayDashboard {
    pub today_water_total: f64,
    pub today_calories_total: f64,
    pub today_protein_total: f64,
    pub today_carb_total: f64,
    pub today_fat_total: f64,
    pub today_sport_duration: i64,
    pub today_sport_count: i64,
    pub open_todos: i64,
    pub today_bilan_count: i64,
    pub today_mood_count: i64,
    pub today_sleep_count: i64,
}