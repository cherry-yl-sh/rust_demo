pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Post {
    title: String,
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.title)
    }
}
pub struct Twitter {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Twitter {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}