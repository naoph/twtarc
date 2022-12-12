CREATE TABLE tweets (
    serialid BIGSERIAL PRIMARY KEY,
    author_id text NOT NULL,
    reply_settings text NOT NULL,
    lang text NOT NULL,
    id text UNIQUE NOT NULL,
    text text NOT NULL,
    retweet_count int NOT NULL,
    reply_count int NOT NULL,
    like_count int NOT NULL,
    quote_count int NOT NULL,
    possibly_sensitive boolean NOT NULL,
    created_at timestamp with time zone NOT NULL,
    source text NOT NULL,
    conversation_id text NOT NULL,
    retweeted text,
    quoted text,
    replied_to text,
    media bigint[] NOT NULL,
    urls bigint[] NOT NULL
);

CREATE TABLE urls (
    serialid BIGSERIAL PRIMARY KEY,
    start int NOT NULL,
    end_ int NOT NULL,
    url text NOT NULL,
    expanded_url text NOT NULL,
    display_url text NOT NULL,
    media_key text,
    status int,
    title text,
    description text,
    unwound_url text
);

CREATE TABLE media (
    serialid BIGSERIAL PRIMARY KEY,
    media_key text NOT NULL,
    type text NOT NULL,
    url text,
    alt_text text,
    duration_ms int
);

CREATE TABLE users (
    serialid BIGSERIAL PRIMARY KEY,
    id text UNIQUE NOT NULL,
    username text UNIQUE NOT NULL,
    displayname text NOT NULL,
    subscribed boolean NOT NULL,
    updated timestamp with time zone
);
