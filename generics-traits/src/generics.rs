
// life time needed due to 2 refs
pub fn biggest<'a,T: PartialOrd>(list1: &'a [T], list2: &'a [T]) -> &'a T {

    // no & needed here since listX is already a ref
    let biggest1 = biggest_helper(list1);
    let biggest2 = biggest_helper(list2);

    if biggest1 > biggest2 {
        biggest1
    } else {
        biggest2
    }
}

// no lifetime needed because of lifetime ellision rules
fn biggest_helper<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in list {
        if *item > *biggest {
            biggest = item;
        }
    }
    biggest
}