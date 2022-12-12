use chrono::{DateTime, Utc};

use crate::schema::*;

#[derive(Debug, Queryable)]
pub struct DbTweet {
    pub serialid: i64,
    pub author_id: String,
    pub reply_settings: String,
    pub lang: String,
    pub id: String,
    pub text: String,
    pub retweet_count: i32,
    pub reply_count: i32,
    pub like_count: i32,
    pub quote_count: i32,
    pub possibly_sensitive: bool,
    pub created_at: DateTime<Utc>,
    pub source: String,
    pub conversation_id: String,
    pub retweeted: Option<String>,
    pub quoted: Option<String>,
    pub replied_to: Option<String>,
    pub media: Vec<i64>,
    pub urls: Vec<i64>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tweets)]
pub struct InsTweet<'a> {
    pub author_id: &'a str,
    pub reply_settings: &'a str,
    pub lang: &'a str,
    pub id: &'a str,
    pub text: &'a str,
    pub retweet_count: i32,
    pub reply_count: i32,
    pub like_count: i32,
    pub quote_count: i32,
    pub possibly_sensitive: bool,
    pub created_at: DateTime<Utc>,
    pub source: &'a str,
    pub conversation_id: &'a str,
    pub retweeted: Option<&'a str>,
    pub quoted: Option<&'a str>,
    pub replied_to: Option<&'a str>,
    pub media: Vec<i64>,
    pub urls: Vec<i64>,
}

#[derive(Debug, Queryable)]
pub struct DbUser {
    pub serialid: i64,
    pub id: String,
    pub username: String,
    pub displayname: String,
    pub subscribed: bool,
    pub updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct InsUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub displayname: &'a str,
    pub subscribed: bool,
    pub updated: Option<DateTime<Utc>>,
}
