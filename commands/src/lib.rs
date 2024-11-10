use core::runner::google_search;
use std::process::Command;

use common::AppData;

mod core;
pub fn parse_command(query: &str, app: Option<&AppData>) {
    if query.starts_with("/web") {
        google_search(query.replace("/web", "").trim()).expect("failed to open the specified url");
        return;
    }
    //incase it's a general search to open an app
    match app {
        Some(data) => {
            println!("app path: {}", &data.path);
            Command::new(&data.path)
                .spawn()
                .expect("failed to launch app");
        }
        None => println!("Error path for app not found"),
    }
}
