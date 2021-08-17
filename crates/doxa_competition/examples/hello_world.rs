use doxa_competition::client::Competition;

async fn test() {
    println!("test");
}

fn main() {
    let competition = Competition::new("example", test, test);
}
