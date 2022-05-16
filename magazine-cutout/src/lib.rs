use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut counter: HashMap<&str, i32> = HashMap::new();

    for key in magazine {
        *counter.entry(key).or_insert(0) += 1;
    }

    note.iter().all(|key| {
        if !counter.contains_key(key) {
            return false;
        }

        if counter[key] < 1 {
            return false;
        }

        counter.insert(key, counter[key] - 1);

        true
    })
}
