use clap::Parser;
use time::macros::datetime;
use twitter_v2::{authorization::BearerToken, TwitterApi};

#[derive(Parser, Debug)]
struct ByeElonArgs {
    token: String,
    start_tweet_id: u64,
}

#[tokio::main]
async fn main() {
    let args = ByeElonArgs::parse();
    println!("{:?}", args);

    let auth = BearerToken::new(args.token);

    let old_tweets = TwitterApi::new(auth)
        .with_user_ctx()
        .await
        .unwrap()
        .get_my_tweets()
        .start_time(datetime!(2010 - 11 - 06 00:00:00 +0))
        .max_results(20)
        .send()
        .await
        .unwrap()
        .into_data();

    old_tweets.unwrap().iter().for_each(|tweet| {
        println!("Text: {:?}", tweet.text);
        println!("ID: {:?}", tweet.id);
        println!("{:?}", tweet.created_at);
    });

    // println!("{:?}", old_tweets);
}
