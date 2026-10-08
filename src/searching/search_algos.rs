pub fn search_key<T>(v: &[T], key: T) -> Option<usize>
where
    T: PartialEq,
{
    if v.len() == 0 {
        return None;
    } else {
        if v[0] == key {
            Some(0)
        } else {
            search_key(&v[1..], key).map(|i| i + 1)
        }
    }
}

pub fn search_sorted_key<T>(v: &[T], key: T) -> Option<usize>
where
    T: PartialEq + Ord,
{
    if v.len() == 0 {
        return None;
    } else {
        let m = v.len() / 2;
        if v[m] == key {
            return Some(m);
        } else if v[m] > key {
            return search_sorted_key(&v[..m], key);
        } else {
            return search_sorted_key(&v[m + 1..], key).map(|i| i + m + 1);
        }
    }
}

pub fn search_max<T>(v: &[T], i: usize, j: usize) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if v.is_empty() || i > j || j >= v.len() {
        return None;
    } else {
        if i == j {
            return Some(v[i]);
        } else {
            let m = (i + j) / 2;
            let m1 = search_max(v, i, m);
            let m2 = search_max(v, m + 1, j);
            match (m1, m2) {
                (Some(val1), Some(val2)) => {
                    if val1 > val2 {
                        Some(val1)
                    } else {
                        Some(val2)
                    }
                }
                (Some(val1), None) => Some(val1),
                (None, Some(val2)) => Some(val2),
                (None, None) => None,
            }
        }
    }
}

pub fn search_max_subarray<T: Num + PartialOrd + Copy>(
    v: &[T],
    i: usize,
    j: usize,
) -> (usize, usize, T) {
    if i == j {
        return (i, j, v[i]);
    } else {
        let m = (i + j) / 2;
        let (l, fl, sl) = search_max_subarray(v, i, m);
        let (r, fr, sr) = search_max_subarray(v, m + 1, j);
        let (c, cr, sc) = search_max_crossing_subarray(v, i, m, j);

        if sl >= sr && sl >= sc {
            (l, fl, sl)
        } else if sr >= sl && sr >= sc {
            (r, fr, sr)
        } else {
            (c, cr, sc)
        }
    }
}

use num::Num;

pub fn search_max_crossing_subarray<T: Num + PartialOrd + Copy>(
    v: &[T],
    i: usize,
    m: usize,
    j: usize,
) -> (usize, usize, T) {
    let mut left_max_sum = None;
    let mut max_left_idx = m;
    let mut current_sum = T::zero();

    for k in (i..=m).rev() {
        current_sum = current_sum + v[k];

        if left_max_sum.is_none() || current_sum > left_max_sum.unwrap() {
            left_max_sum = Some(current_sum);
            max_left_idx = k;
        }
    }

    let mut right_max_sum = None;
    let mut max_right_idx = m + 1;
    current_sum = T::zero();

    for k in m + 1..=j {
        current_sum = current_sum + v[k];

        if right_max_sum.is_none() || current_sum > right_max_sum.unwrap() {
            right_max_sum = Some(current_sum);
            max_right_idx = k;
        }
    }

    let final_left_sum = left_max_sum.unwrap_or(T::zero());
    let final_right_sum = right_max_sum.unwrap_or(T::zero());

    (
        max_left_idx,
        max_right_idx,
        final_left_sum + final_right_sum,
    )
}
