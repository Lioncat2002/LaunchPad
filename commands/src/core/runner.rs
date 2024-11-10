use std::io::Error;

pub fn google_search(uri:&str)->Result<(),Error>{
    open::that("https://www.google.com/search?q=".to_owned()+uri)
}