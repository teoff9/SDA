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

pub fn insertion_sort<T>(v: &mut [T])
where
    T: Ord,
{
    let n = v.len();
    for i in 1..n {
        let mut k = i;
        for j in (0..k).rev() {
            if v[j] > v[k] {
                v.swap(j, k);
                k = j;
            } else {
                break;
            }
        }
    }
}
