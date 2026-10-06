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
