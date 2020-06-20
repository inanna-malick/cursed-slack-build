use slack_api::default_client;
use slack_api::users_profile::{get, GetRequest};
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let token = std::env::var("SLACK_TOKEN").expect("expected env var SLACK_TOKEN for build.rs");
    let slack_uid = std::env::var("SLACK_UID").expect("expected env var SLACK_UID for build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("slack_blobs_output_dir");
    let _ = fs::remove_dir_all(&dest_path); // may already exist, nuke if that is the case
    fs::create_dir(&dest_path).unwrap();

    println!("dest path: {:?}", &dest_path);

    let f_dest_path = Path::new(&out_dir).join("slack_blob.rs");
    let mut f = fs::File::create(&f_dest_path).unwrap();

    let req = GetRequest {
        user: Some(&slack_uid),
        include_labels: None,
    };
    let client = default_client().unwrap();
    let user_profile = get(&client, &token, &req).expect("error unwrapping slack resp");

    println!("user profile: {:?}", user_profile);

    let display_name = user_profile
        .profile
        .expect("profile field not present")
        .display_name
        .expect("display name not present");

    let output = format!(
        r#"static BUILT_BY: &'static str = "{}";"#,
        display_name
    );

    f.write_all(&output.into_bytes()).unwrap();

    // panic!("afaik only way to get println output from build.rs is to fail here");
}
