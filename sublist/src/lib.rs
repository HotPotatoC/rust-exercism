#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let is_sublist = |v: &[T], u: &[T]| v.is_empty() || u.windows(v.len()).any(|x| x == v);

    if first_list == second_list {
        Comparison::Equal
    } else if first_list.len() < second_list.len() && is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
