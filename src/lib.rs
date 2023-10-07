//! Insert sort algorithm practice implementations.

use std::fmt::Debug;

fn nondecreasing_insertion_sort<T>(source: &[T]) -> Vec<T>
where
    T: PartialEq + PartialOrd + Clone,
{
    if let Some(head) = source.first() {
        let mut dest = Vec::with_capacity(source.len());
        dest.insert(0, head.clone());
        for source_item in source.iter().skip(1) {
            for (index, dest_item) in dest.iter().enumerate() {
                if source_item < dest_item {
                    dest.insert(index, source_item.clone());
                    break;
                }
                if index == dest.len() - 1 {
                    dest.push(source_item.clone());
                    break;
                }
            }
        }
        dest
    } else {
        source.to_vec()
    }
}

fn insertion_sort_in_place<T>(source: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Clone + Debug,
{
    let len = source.len();
    for pos in 1..len {
        let mut pos = pos;
        while pos > 0 && source[pos] < source[pos - 1] {
            source.swap(pos, pos - 1);
            pos -= 1;
        }
    }
}

fn nonincreasing_insertion_sort<T>(source: &[T]) -> Vec<T>
where
    T: PartialEq + PartialOrd + Clone,
{
    if let Some(head) = source.first() {
        let mut dest = Vec::with_capacity(source.len());
        dest.insert(0, head.clone());
        for source_item in source.iter().skip(1) {
            for (index, dest_item) in dest.iter().enumerate() {
                if source_item > dest_item {
                    dest.insert(index, source_item.clone());
                    break;
                }
                if index == dest.len() - 1 {
                    dest.push(source_item.clone());
                    break;
                }
            }
        }
        dest
    } else {
        source.to_vec()
    }
}

const UNSORTED: &[u32; 6] = &[31, 41, 59, 26, 41, 58];

#[test]
fn sort_nondec_vec() {
    const SORTED: &[u32; 6] = &[26, 31, 41, 41, 58, 59];
    assert_eq!(nondecreasing_insertion_sort(UNSORTED), SORTED);
}

#[test]
fn sort_vec_in_place() {
    let mut unsorted = UNSORTED.to_vec();
    insertion_sort_in_place(&mut unsorted);
    const SORTED: &[u32; 6] = &[26, 31, 41, 41, 58, 59];
    assert_eq!(unsorted, SORTED);
}

#[test]
fn sort_noninc_vec() {
    const SORTED: &[u32; 6] = &[59, 58, 41, 41, 31, 26];
    assert_eq!(nonincreasing_insertion_sort(UNSORTED), SORTED);
}
