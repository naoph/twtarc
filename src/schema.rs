table! {
    media (serialid) {
        serialid -> Int8,
        media_key -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        url -> Nullable<Text>,
        alt_text -> Nullable<Text>,
        duration_ms -> Nullable<Int4>,
    }
}

table! {
    tweets (serialid) {
        serialid -> Int8,
        author_id -> Text,
        reply_settings -> Text,
        lang -> Text,
        id -> Text,
        text -> Text,
        retweet_count -> Int4,
        reply_count -> Int4,
        like_count -> Int4,
        quote_count -> Int4,
        possibly_sensitive -> Bool,
        created_at -> Timestamptz,
        source -> Text,
        conversation_id -> Text,
        retweeted -> Nullable<Text>,
        quoted -> Nullable<Text>,
        replied_to -> Nullable<Text>,
        media -> Array<Int8>,
        urls -> Array<Int8>,
    }
}

table! {
    urls (serialid) {
        serialid -> Int8,
        start -> Int4,
        end_ -> Int4,
        url -> Text,
        expanded_url -> Text,
        display_url -> Text,
        media_key -> Nullable<Text>,
        status -> Nullable<Int4>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        unwound_url -> Nullable<Text>,
    }
}

table! {
    users (serialid) {
        serialid -> Int8,
        id -> Text,
        username -> Text,
        displayname -> Text,
        subscribed -> Bool,
        updated -> Nullable<Timestamptz>,
    }
}

allow_tables_to_appear_in_same_query!(
    media,
    tweets,
    urls,
    users,
);
