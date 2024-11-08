use strsim::levenshtein;

pub fn similarity_search<'a>(apps:Vec<&'a str>,query:&str)->Vec<(&'a str,usize)>{
    let mut similarities:Vec<(&str,usize)>=apps.into_iter().map(|s| (s,levenshtein(query, &s))).collect();
    similarities.sort_by(|a, b| a.1.cmp(&b.1));
    similarities
}