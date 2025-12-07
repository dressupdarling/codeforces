// time O(r log r)
// mem O(m)

pub fn count_total_fresh_ids(input: &str) -> u64 {
    let mut ranges: Vec<(u64, u64)> = input
        .trim()
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || !line.contains('-') {
                return None;
            }
            let mut parts = line.split('-');
            let a = parts.next()?.parse::<u64>().ok()?;
            let b = parts.next()?.parse::<u64>().ok()?;
            Some((a, b))
        })
        .collect();

    if ranges.is_empty() {
        return 0;
    }

    ranges.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (start, end) in ranges {
        if let Some((_last_start, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    merged.iter().map(|(start, end)| end - start + 1).sum()
}
