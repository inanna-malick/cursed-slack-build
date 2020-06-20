include!(concat!(env!("OUT_DIR"), "/slack_blob.rs"));

fn main() {
    println!("this app was built by slack user {}!", BUILT_BY);
}
