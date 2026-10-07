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

pub fn merge<T>(v: &mut [T], i: usize, m: usize, j: usize)
where
    T: Ord + Copy,
{
    let n = j - i + 1;
    let mut w: Vec<T> = Vec::with_capacity(n);
    let mut x = i;
    let mut y = m + 1;

    while x <= m && y <= j {
        if v[x] <= v[y] {
            w.push(v[x]);
            x += 1;
        } else {
            w.push(v[y]);
            y += 1;
        }
    }

    w.extend_from_slice(&v[x..=m]);
    w.extend_from_slice(&v[y..=j]);
    v[i..=j].copy_from_slice(&w);
}
