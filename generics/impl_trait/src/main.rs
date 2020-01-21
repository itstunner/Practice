use trai_library :: {Summary,Tweet, notify, returns_summarizable};
fn main() {

let tweet = Tweet{
    username: String :: from("horse_ebooks"),
    content: String :: from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
    };
        
    println!("1 new tweet: {}", tweet.summarize());

    let tweet1 = Tweet{
        username: String :: from("Guide book"),
        content: String :: from("You are genius by the way"),
        reply: true,
        retweet: true,
    };

    let name = String :: from("Fawad");

    println!("retweet is: {:?}", notify(tweet1));

let return_trait = returns_summarizable();
println!("{}", &return_trait.username);
}

