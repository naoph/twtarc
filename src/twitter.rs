use snafu::{Snafu, ResultExt};
use twitter_v2::TwitterApi;

#[derive(Debug)]
pub struct BasicUserInfo {
    pub id: String,
    pub username: String,
    pub displayname: String,
}

pub struct Twitter {
    auth: twitter_v2::authorization::BearerToken,
}

impl Twitter {
    pub fn new(auth: twitter_v2::authorization::BearerToken) -> Twitter {
        Twitter { auth }
    }

    pub async fn user_by_username(&self, username: &str) -> Result<BasicUserInfo, TwtErr> {
        let response = TwitterApi::new(self.auth.clone())
            .get_user_by_username(username)
            .send()
            .await
            .context(ApiSnafu)?;
        match response.data.clone() {
            None => Err(TwtErr::NotFoundError),
            Some(u) => Ok(BasicUserInfo {
                id: u.id.to_string(),
                username: u.username,
                displayname: u.name
            }),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum TwtErr {
    ApiError { source: twitter_v2::Error },
    NotFoundError,
}
