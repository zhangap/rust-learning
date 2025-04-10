use trait_demo::Summary;
use trait_demo::Tweet;
fn main() {
    let tweet = Tweet{
        username: String::from("特朗普"),
        content: String::from("特朗普拉开了关税大战"),
        reply:false,
        retweet:false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
