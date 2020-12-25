use super::Sorter;

pub struct Insert;

impl Sorter for Insert {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let mut i = unsorted;
            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut numbers = vec![4, 1, 5, 2, 6, 3];
    Insert::sort(&mut numbers);
    assert_eq!(numbers, &[1, 2, 3, 4, 5, 6]);
}
