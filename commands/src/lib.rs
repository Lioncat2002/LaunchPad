use core::runner::google_search;


mod core;
pub fn parse_command(query:&str){

    if query.starts_with("/web"){
      google_search(query.replace("/web", "").trim()).expect("failed to open the specified url");
    }
}
