use crate::sorting::utils::find_min;

pub fn selection_sort<T>(v: &mut [T])
where
    T: Ord,
{
    for i in 0..v.len() {
        let min_idx = find_min(v, i, v.len() - 1);
        v.swap(i, min_idx);
    }
}
