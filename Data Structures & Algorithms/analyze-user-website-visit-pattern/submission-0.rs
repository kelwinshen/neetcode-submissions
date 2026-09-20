use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn most_visited_pattern(
        username: Vec<String>,
        timestamp: Vec<i32>,
        website: Vec<String>,
    ) -> Vec<String> {
        // The input is given as three PARALLEL arrays -- username[i], timestamp[i],
        // website[i] together describe ONE visit event. First job: reorganize this
        // into "each user's own browsing history", since the problem is really asking
        // about patterns WITHIN one user's timeline, not across the raw flat list.
        let mut user_visits: HashMap<String, Vec<(i32, String)>> = HashMap::new();
        for i in 0..username.len() {
            user_visits
                .entry(username[i].clone())
                .or_insert_with(Vec::new)
                .push((timestamp[i], website[i].clone()));
        }

        // The problem cares about ORDER ("in the same order they appeared"), so each
        // user's visits must be sorted by when they actually happened, not by the
        // order they showed up in the input arrays.
        for visits in user_visits.values_mut() {
            visits.sort_by_key(|&(t, _)| t);
        }

        // This will hold: for every possible 3-website pattern, how many DISTINCT
        // USERS had that pattern appear as a subsequence somewhere in their history.
        let mut global_counts: HashMap<(String, String, String), i32> = HashMap::new();

        for visits in user_visits.values() {
            let sites: Vec<&String> = visits.iter().map(|(_, w)| w).collect();
            let n = sites.len();

            // "the pattern must appear as a SUBSEQUENCE" -- meaning the 3 websites
            // just need to occur in that relative order somewhere in this user's
            // timeline, not consecutively. So: try every way to pick 3 positions
            // i < j < k out of this user's visits, and read off the pattern at
            // those positions.
            let mut this_user_patterns: HashSet<(String, String, String)> = HashSet::new();
            for i in 0..n {
                for j in (i + 1)..n {
                    for k in (j + 1)..n {
                        let pattern = (sites[i].clone(), sites[j].clone(), sites[k].clone());
                        this_user_patterns.insert(pattern);
                        // HashSet here means: if THIS SAME user can reach the same
                        // pattern through more than one (i,j,k) choice, it still only
                        // counts ONCE for this user -- "score = number of USERS", not
                        // number of ways to find the pattern.
                    }
                }
            }

            // Now that this user's patterns are deduplicated, each one contributes
            // exactly one vote to the global tally.
            for pattern in this_user_patterns {
                *global_counts.entry(pattern).or_insert(0) += 1;
            }
        }

        // Find the pattern with the highest score. On a tie, the problem wants the
        // LEXICOGRAPHICALLY SMALLEST pattern -- Rust compares tuples of Strings
        // left-to-right automatically, so `pattern < bp` already does exactly that.
        let mut best_pattern: Option<(String, String, String)> = None;
        let mut best_count = 0;

        for (pattern, count) in &global_counts {
            if *count > best_count
                || (*count == best_count && best_pattern.as_ref().map_or(true, |bp| pattern < bp))
            {
                best_count = *count;
                best_pattern = Some(pattern.clone());
            }
        }

        let (a, b, c) = best_pattern.unwrap();
        vec![a, b, c]
    }
}