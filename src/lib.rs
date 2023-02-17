pub mod double_linked_list;
#[cfg(test)]
mod tests;

#[test]
fn run_part1() {
    use crate::tests::{test_back, test_front, test_len};
    test_front();
    test_back();
    test_len();

    use crate::tests::{test_for_loop, test_iter, test_iter_mut, test_rev_for_loop};
    test_iter();
    test_iter_mut();
    test_for_loop();
    test_rev_for_loop();
}

#[test]
fn run_part2() {
    use crate::tests::{
        test_contains, test_find_mut, test_get, test_insert, test_remove, test_split,
    };
    test_get();
    test_remove();
    test_insert();
    test_contains();
    test_split();
    test_find_mut();
}

#[test]
fn run_part3() {
    use crate::tests::{test_merge_sort1, test_merge_sort2, test_merge_sort3};
    test_merge_sort1();
    test_merge_sort2();
    test_merge_sort3();
}