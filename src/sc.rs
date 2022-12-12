use comfy_table::Table;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use futures::FutureExt;

use crate::models;
use crate::schema;
use crate::twitter::{Twitter, TwtErr};
use crate::PgPool;

pub fn sync(_twt: Twitter, _pool: PgPool) {
    println!("Syncing...");
}

pub async fn user_list(pool: PgPool) {
    // Load list of users
    let mut conn = pool.get()
        .await
        .expect("Failed to connect to database");
    let user_list: Vec<models::DbUser> = schema::users::table
        .order(schema::users::username.asc())
        .load(&mut conn)
        .await
        .expect("Failed to list users");
    let mut vec_vec = Vec::new();
    for row in user_list.into_iter() {
        let subscribed = String::from(if row.subscribed { "Yes" } else { "No" });
        let updated = match row.updated {
            None => "Never".to_string(),
            Some(t) => t.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        vec_vec.push(vec![row.username, row.displayname, subscribed, updated])
    }

    // Render table
    let mut table = Table::new();
    table.load_preset(comfy_table::presets::ASCII_MARKDOWN);
    table.set_header(vec!["Username", "Display name", "Subscribed", "Last updated"]);
    table.add_rows(vec_vec);
    println!("{table}")
}

pub async fn user_add(twt: Twitter, pool: PgPool, url: url::Url) {
    // Get username from URL
    let path_split: Vec<&str> = url.path().split("/").collect();
    if path_split.len() < 2 || url.host_str() != Some("twitter.com") {
        eprintln!("This URL is not a valid Twitter profile");
        return;
    }
    let username = path_split[1];

    // Load user info
    let user = twt.user_by_username(username).await;
    if let Err(err) = user {
        match err {
            TwtErr::NotFoundError => {
                eprintln!("User @{} does not exist", username);
            }
            TwtErr::ApiError { source } => {
                eprintln!("An unknown error occurred: {source}");
            }
        }
        return;
    }
    let user = user.unwrap();

    // Insert if applicable
    let mut conn = pool.get()
        .await
        .expect("Failed to connect to database");
    let tr = conn.transaction::<_, Error, _>(|conn| async move {
        let existing_user = schema::users::table
            .filter(schema::users::username.eq(&user.username))
            .first::<models::DbUser>(conn)
            .await
            .optional()?;
        if let Some(eu) = existing_user {
            // User exists, ensure they're enabled
            if eu.subscribed {
                println!("Subscription to {} is already enabled", eu.username);
            } else {
                diesel::update(schema::users::table.find(eu.serialid))
                    .set(schema::users::subscribed.eq(true))
                    .execute(conn)
                    .await?;
                println!("Subscription to {} has been re-enabled", eu.username);
            }
        } else {
            // User doesn't exist, insert them
            let ins_user = models::InsUser {
                id: &user.id,
                username: &user.username,
                displayname: &user.displayname,
                subscribed: true,
                updated: None,
            };
            diesel::insert_into(schema::users::table)
                .values(&ins_user)
                .execute(conn)
                .await?;
            println!("Subscribed to new user @{} / {} / {}", user.username, user.displayname, user.id);
        }
        Ok(())
    }.boxed()).await;
    if let Err(e) = tr {
        eprintln!("Unknown error updating database: {e}");
    }
}

pub fn insert(_twt: Twitter, _pool: PgPool, url: url::Url) {
    println!("Inserting tweet {url}");
}
