use std::cmp::Ordering;
use std::cmp::Ord;

pub fn find<C, T>(collection: C, key: T) -> Option<usize>
where
    C: AsRef<[T]>,
    T: Ord,
{
    let array = collection.as_ref();
    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match array[mid].cmp(&key) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
            Ordering::Equal => return Some(mid),
        }
    }

    None
}
