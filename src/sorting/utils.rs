//returns the index to the minimum element in an array from idx to fdx
pub fn find_min<T>(v: &[T], idx: usize, fdx: usize) -> usize
where
    T: Ord,
{
    let mut min_idx = idx;
    for i in idx..=fdx {
        if v[i] < v[min_idx] {
            min_idx = i;
        }
    }
    min_idx
}
