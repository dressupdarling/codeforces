// time O(r log r + n log m)
// mem O(n + m)

pub fn count_fresh_ids(input: &str) -> usize {
    let mut sections = input.trim().split("\n\n");
    let ranges_section = sections.next().unwrap();
    let ids_section = sections.next().unwrap();
    let mut ranges: Vec<(u64, u64)> = ranges_section
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('-');
            Some((
                parts.next()?.parse::<u64>().ok()?,
                parts.next()?.parse::<u64>().ok()?,
            ))
        })
        .collect();

    ranges.sort_by_key(|&(start, _)| start);
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some((_s, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let ids: Vec<u64> = ids_section
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    ids.iter()
        .filter(|&&id| {
            merged.binary_search_by(|&(start, end)| {
                if id < start {
                    std::cmp::Ordering::Greater
                } else if id > end {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            }).is_ok()
        })
        .count()
}
