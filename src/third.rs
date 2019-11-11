use super::SortOrder;
use std::cmp::Ordering;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        return Ok(());
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len():{}",
            x.len()
        ))
    }
}

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}
fn do_sort<T, F>(x: &mut [T], up: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true, comparator);
        do_sort(&mut x[mid_point..], false, comparator);
        sub_sort(x, up, comparator);
    }
}
fn sub_sort<T, F>(x: &mut [T], up: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, up, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up, comparator);
        sub_sort(&mut x[mid_point..], up, comparator);
    }
}
fn compare_and_swap<T, F>(x: &mut [T], up: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let mid_point = x.len() / 2;
    let swap_condition = if up {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    for i in 0..mid_point {
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sort, sort_by};
    use crate::SortOrder::*;

    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }
    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];
        assert!(sort(&mut x, &Ascending).is_err());
    }

    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }
    impl Student {
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age,
            }
        }
    }

    #[test]
    fn sort_students_by_age_ascending() {
        let t1 = Student::new("Taro1", "Tanaka1", 16);
        let t2 = Student::new("Taro2", "Tanaka2", 14);
        let t3 = Student::new("Taro3", "Tanaka3", 15);
        let t4 = Student::new("Taro4", "Tanaka4", 17);
        let mut x = vec![&t1, &t2, &t3, &t4];
        let expected = vec![&t2, &t3, &t1, &t4];
        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)), Ok(()));
        assert_eq!(x, expected);
    }
}
