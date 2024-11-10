use common::AppData;
use strsim::levenshtein;

pub fn similarity_search(apps: &[AppData], query: &str) -> Vec<AppData> {
    let mut similarities: Vec<(AppData, usize)> = apps
        .iter()
        .map(|s| (s.clone(), levenshtein(query, &s.name)))
        .collect();

    similarities.sort_by(|a, b| a.1.cmp(&b.1));

    similarities.into_iter().map(|(s, _)| s).collect()
}
