// use crate::double_linked_list_example::{LinkedList, MergeSort};
use crate::double_linked_list::{LinkedList, MergeSort};
use rand::{distributions::Uniform, Rng};
#[test]
pub fn test_front() {
    // 测试 front,push_front,pop_front
    let mut list = LinkedList::new();
    assert_eq!(list.front(), None);
    for i in 0..1000 {
        list.push_front(i);
        assert_eq!(list.front(), Some(&i));
        if i % 2 == 0 {
            assert_eq!(list.pop_front(), Some(i));
        }
    }
    for i in (0..1000).rev() {
        if i % 2 != 0 {
            assert_eq!(list.pop_front(), Some(i));
        }
    }
}

#[test]
pub fn test_back() {
    // 测试 back,push_back,pop_back
    let mut list = LinkedList::new();
    assert_eq!(list.back(), None);
    for i in 0..1000 {
        list.push_back(i);
        assert_eq!(list.back(), Some(&i));
        if i % 2 == 0 {
            assert_eq!(list.pop_back(), Some(i));
        }
    }
    for i in (0..1000).rev() {
        if i % 2 != 0 {
            assert_eq!(list.pop_back(), Some(i));
        }
    }
}

#[test]
pub fn test_len() {
    // 测试 len
    let mut list = LinkedList::new();
    assert_eq!(list.len(), 0);
    let mut len = 0;
    for i in 0..500 {
        list.push_back(i);
        len += 1;
        assert_eq!(list.len(), len);
    }
    for i in 0..500 {
        list.push_front(i);
        len += 1;
        assert_eq!(list.len(), len);
    }
}

#[test]
pub fn test_iter() {
    // 测试 iter
    let mut list = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    let mut iter = list.iter();
    for i in 0..1000 {
        assert_eq!(iter.next(), Some(&i));
    }
}

#[test]
pub fn test_iter_mut() {
    // 测试 iter_mut
    let mut list = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    let mut iter = list.iter_mut();
    for i in 0..1000 {
        let mut j = i;
        assert_eq!(iter.next(), Some(&mut j));
    }
}

#[test]
pub fn test_for_loop() {
    // 测试遍历
    let mut list = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    for (i, j) in list.iter().enumerate() {
        assert_eq!(i, *j);
    }
    for (i, j) in list.iter_mut().enumerate() {
        *j = i * 2;
    }
    for (i, j) in list.iter().enumerate() {
        assert_eq!(i * 2, *j);
    }
}

#[test]
pub fn test_rev_for_loop() {
    // 测试双向迭代器
    let mut list = LinkedList::new();
    for i in (0..1000).rev() {
        list.push_back(i);
    }
    for (i, j) in list.iter().rev().enumerate() {
        assert_eq!(i, *j);
    }
}
#[test]
pub fn test_get() {
    let mut list = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    for i in 0..1000 {
        assert_eq!(list.get(i), &i);
    }
}

#[test]
pub fn test_insert() {
    let mut list = LinkedList::new();
    list.push_front(0);
    list.push_back(20);
    for i in 1..20 {
        list.insert(i, i);
    }
    for (i, j) in list.iter().enumerate() {
        assert_eq!(i, *j);
    }
}
#[test]
pub fn test_remove() {
    let mut list = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    for i in 0..(1000 - 42) {
        assert_eq!(list.remove(42), i + 42);
        assert_eq!(list.len(), 999 - i);
    }
}

#[test]
pub fn test_contains() {
    let mut list = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    for i in 0..1000 {
        assert!(list.contains(&i));
    }
}

#[test]
pub fn test_find_mut() {
    // 原来有错的测试程序
    // let mut list = LinkedList::new();
    // let rand_pos: i64 = rand::thread_rng().gen_range(0..1000);
    // // FIXME: 此处应该注意生成的input不能产生相同的随机数
    // let range = Uniform::from(100..1000000);
    // let input: Vec<i64> = rand::thread_rng().sample_iter(&range).take(1000).collect();
    // list.extend(input.iter().cloned());
    // for i in 0..1000 {
    //     let v = input[((i + rand_pos) % 1000) as usize];
    //     list.find_mut(|x| *x == v).map(|x| *x = i as i64);
    // }
    // let mut ans = 1000 - rand_pos;
    // while let Some(x) = list.pop_front() {
    //     assert_eq!(x, ans);
    //     ans = (ans + 1) % 1000;
    // }
    use std::collections::HashSet;
    use rand::distributions::Uniform;
    use rand::Rng;
    use rand::prelude::Distribution;

    let mut list = LinkedList::new();
    let rand_pos: i64 = rand::thread_rng().gen_range(0..1000);

    // FIXME: 此处生成的range范围应该和下标范围不能有重叠
    // let range = Uniform::from(100..1000000);
    let range = Uniform::from(1000..1000000);
    
    // FIXME: 生成没有重复的input
    // let input: Vec<i64> = rand::thread_rng().sample_iter(&range).take(1000).collect();
    let mut input_set: HashSet<i64> = HashSet::new();
    let mut rng = rand::thread_rng();
    while input_set.len() != 1000 {
        input_set.insert(range.sample(&mut rng));
    }
    let input: Vec<i64> = input_set.into_iter().collect();
    

    list.extend(input.iter().cloned());
    
    for i in 0..1000 {
        let v = input[((i + rand_pos) % 1000) as usize];
        list.find_mut(|x| *x == v).map(|x| *x = i as i64);
    }

    let mut ans = 1000 - rand_pos;
    let mut index: usize = 0;
    while let Some(x) = list.pop_front() {
        if x != ans {
            println!("x: {}, ans: {}, index: {}", x, ans, index);
        }
        index += 1_usize;
        ans = (ans + 1) % 1000;
    }
}

#[test]
pub fn test_split() {
    {
        let mut list = LinkedList::new();
        for i in 0..1000 {
            list.push_back(i);
        }
        let list2 = list.split_off(500);
        assert!(list2.len() == 500);
        assert!(list.len() == 500);
        for (i, j) in list.iter().enumerate() {
            assert_eq!(i, *j);
        }
        for (i, j) in list2.iter().enumerate() {
            assert_eq!(i + 500, *j);
        }
        let mut list3 = list.split_off(1);
        let _ = list3.split_off(498);
        for (i, j) in (1..498).zip(list3.iter()) {
            assert_eq!(i, *j);
        }
    }
    {
        let mut list = LinkedList::new();
        "Hello,world!".chars().for_each(|c| list.push_back(c));
        let part1 = "Hello,".chars().collect::<Vec<_>>();
        let part2 = "world!".chars().collect::<Vec<_>>();
        let list2 = list.split_off(6);
        for (i, j) in list.iter().zip(part1.iter()) {
            assert_eq!(*i, *j);
        }
        for (i, j) in list2.iter().zip(part2.iter()) {
            assert_eq!(*i, *j);
        }
    }
}

#[test]
pub fn test_merge_sort1() {
    let input = vec![
        58, 14, 2, 87, 35, 29, 86, 60, 52, 45, 97, 90, 78, 37, 32, 89, 1, 27, 25, 34,
    ];
    let mut solution = vec![
        58, 14, 2, 87, 35, 29, 86, 60, 52, 45, 97, 90, 78, 37, 32, 89, 1, 27, 25, 34,
    ];
    solution.sort();
    let mut list = LinkedList::new();
    for i in input.iter() {
        list.push_back(*i);
    }
    list.merge_sort();
    for (i, j) in list.iter().zip(solution.iter()) {
        assert_eq!(*i, *j);
    }
}

#[test]
pub fn test_merge_sort2() {
    {
        let input = vec![1];
        let mut solution = vec![1];
        solution.sort();
        let mut list = LinkedList::new();
        for i in input.iter() {
            list.push_back(*i);
        }
        list.merge_sort();
        for (i, j) in list.iter().zip(solution.iter()) {
            assert_eq!(*i, *j);
        }
    }
    {
        let input = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let mut solution = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        solution.sort();
        let mut list = LinkedList::new();
        for i in input.iter() {
            list.push_back(*i);
        }
        list.merge_sort();
        for (i, j) in list.iter().zip(solution.iter()) {
            assert_eq!(*i, *j);
        }
    }
}

#[test]
pub fn test_merge_sort3() {
    let range = Uniform::from(0..1000000);
    for _ in 0..5 {
        let input: Vec<i64> = rand::thread_rng()
            .sample_iter(&range)
            .take(100000)
            .collect();
        let mut output = input.clone();
        output.sort();
        let mut list = LinkedList::new();
        for i in input.iter() {
            list.push_back(*i);
        }
        list.merge_sort();
        for (i, j) in list.iter().zip(output.iter()) {
            assert_eq!(*i, *j);
        }
    }
}
