use strsim::levenshtein;

pub fn similarity_search<'a>(apps: &Vec<String>, query: &String) -> Vec<String> {
    let mut similarities: Vec<(String, usize)> = apps
        .iter()
        .map(|s| (s.clone(), levenshtein(query, s)))
        .collect();
    
    similarities.sort_by(|a, b| a.1.cmp(&b.1));
    
    similarities.into_iter().map(|(s, _)| s).collect()
}
