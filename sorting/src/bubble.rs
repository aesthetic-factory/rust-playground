use super::Sorter;

pub struct Bubble;

impl Sorter for Bubble {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn it_works_with_super() {
    let mut numbers = vec![4, 1, 5, 2, 6, 3];
    super::sort::<u32, Bubble>(&mut numbers);
    assert_eq!(numbers, &[1, 2, 3, 4, 5, 6]);
}

#[test]
fn it_works() {
    let mut numbers = vec![4, 1, 5, 2, 6, 3];
    Bubble::sort(&mut numbers);
    assert_eq!(numbers, &[1, 2, 3, 4, 5, 6]);
}