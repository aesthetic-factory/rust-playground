mod bubble;
mod insert;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice);
}


fn main() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    struct StdSorter;
    impl Sorter for StdSorter{
        fn sort<T>(slice: &mut [T])
        where T:Ord,{
            slice.sort();
        }
    }
    #[test]
    fn std_sort() {
        let mut numbers = vec![4,1,5,2,6,3];
        sort::<u32,StdSorter>(&mut numbers);
        assert_eq!(numbers, &[1,2,3,4,5,6]);
    }


}