//! Chapter 2.1 Insertion Sort, Linear Search & Binary Addition
//!
//! The pseudo-code for the insertion sort function from the book is as follows:
//! INSERTION-SORT(A, n)
//! 1 for i = 2 to n
//! 2     key = A[i]
//! 3     // Insert A[i] into the sorted subarray A[1 : i - 1]
//! 4     j = i - 1
//! 5     while j > 0 and A[j] > key
//! 6         A[j + 1] = A[j]
//! 7         j = j - 1
//! 8     A[j + 1] = key
//!
//! Exercise 2.1-1:
//! Using Figure 2.2 as a model, illustrate the operation of INSERTION-SORT on the array
//! A = 31, 41, 59, 26, 41, 58
//!
//! Exercise 2.1-2:
//! Rewrite the INSERTION-SORT procedure to sort into nonincreasing instead of
//! nondecreasing order.
//!
//! Exercise 2.1-3:
//! Consider the searching problem:
//!     • Input: A sequence of n numbers A = a(1), a(2), . . . , a(n) and a value v.
//!     • Output: An index i such that v = A[i] or the special value NIL if v does not appear in A.
//! Write pseudocode for linear search, which scans through the sequence, looking for v. Using a
//! loop invariant, prove that your algorithm is correct. Make sure that your loop invariant fulfills
//! the three necessary properties.
//!
//! Exercise 2.1-4:
//! Consider the problem of adding two n-bit binary integers, stored in two n-element arrays A
//! and B. The sum of the two integers should be stored in binary form in an (n + 1)-element
//! array C. State the problem formally and write pseudocode for adding the two integers.

/// Exercise 2.1-1
/// returns a new sorted vector from least to greatest
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

/// Exercise 2.1-1
/// non-decreasing in-place insertion sort
fn insertion_sort_in_place<T>(source: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Clone + std::fmt::Debug,
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

/// Exercise 2.1-2
/// returns a new sorted vector from greatest to least
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

/// Exercise 2.1-3
fn linear_search<T>(source: &[T], target: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    for index in 0..source.len() {
        if source[index] == target {
            return Some(index);
        }
    }
    None
}

/// Exercise 2.1-4
/// 1. Two n-bit binary integers stored in n-element arrays when
/// added together could potentially overflow. In that case, we can add them
/// together safely by storing the result in an n+1 element array.
/// 2. Pseudo-code:
/// - create a new vector with n+1 capacity
/// - apply binary addition to a & b
/// - store their sum in the newly created vector
/// - return sum
///
/// ADD_BINARY_INTEGERS(A, B, n)
/// let sum = Vec::with_capacity(n+1);
/// let i = n - 1; // the index of A & B
/// let mut c = 0; // the overflow
/// for i in (0..=i).rev() {
///     let x = A[i] + B[i];
///     if c == 1 {
///         if x == 0 {
///             sum.insert(0, x + c);
///             c = 0;
///         } else if x == 2 {
///             sum.insert(0, x - c);
///         } else {
///             sum.insert(0, 0);
///         }
///     } else {
///         if x < 2 {
///             sum.insert(0, x);
///         } else {
///             sum.insert(0, 0);
///             c = 1;
///         }
///     }
/// }
/// sum.insert(0, c); // insert the overflow
///
/// sum
fn add_binary_integers(a: &[u8], b: &[u8], n: usize) -> Vec<u8> {
    let mut sum: Vec<u8> = Vec::with_capacity(n + 1);
    let mut c = 0;
    for i in (0..n).rev() {
        let x = a[i] + b[i];
        if c == 1 {
            if x == 0 {
                sum.insert(0, x + c);
                c = 0;
            } else if x == 2 {
                sum.insert(0, x - c);
            } else {
                sum.insert(0, 0);
            }
        } else {
            if x < 2 {
                sum.insert(0, x);
            } else {
                sum.insert(0, 0);
                c = 1;
            }
        }
    }
    sum.insert(0, c);

    sum
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

#[test]
fn element_exists() {
    const ELEMENT: u32 = 31;
    assert_eq!(linear_search(UNSORTED, ELEMENT), Some(0));
}

#[test]
fn element_does_not_exist() {
    const ELEMENT: u32 = 105;
    assert_eq!(linear_search(UNSORTED, ELEMENT), None);
}

#[test]
fn add_binary_ints() {
    const A: &[u8] = &[0, 1, 0, 1];
    const B: &[u8] = &[1, 1, 1, 1];
    assert_eq!(add_binary_integers(A, B, 4), vec![1, 0, 1, 0, 0]);
}
