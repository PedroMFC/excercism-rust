#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let mut result = Comparison::Unequal;
    if first_list == second_list {
        result = Comparison::Equal;
    } else if first_list.len() < second_list.len() {
        result = Comparison::Sublist;
        for i in 0..first_list.len() {
            if first_list[i] != second_list[i] {
                let result_tmp = sublist(first_list, &second_list[1..]);
                if result_tmp == Comparison::Unequal {
                    result = result_tmp;
                };
            }
        }
    } else if first_list.len() > second_list.len() {
        result = Comparison::Superlist;
        for i in 0..second_list.len() {
            if second_list[i] != first_list[i] {
                let result_tmp = sublist(&first_list[1..], second_list);
                if result_tmp == Comparison::Unequal {
                    result = result_tmp;
                };
            }
        }
    }
    result
}
