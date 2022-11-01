use std::collections::HashSet;

fn long_add(v: Vec<usize>, w: Vec<usize>) -> Vec<usize> {
    let (mut i, mut j, mut k): (usize, usize, usize) = (0, 0, 0);
    let (vl, wl) = (v.len(), w.len());
    let mut done: bool = false;

    let l: usize = maxu(vl, wl);

    let mut result: Vec<usize> = vec![0; l];

    while !done {
        let mut s: usize = 0;

        if i < vl {
            s += v[i];
        } else {
            done = true;
        }

        if j < wl {
            s += w[j];
            done = false;
        }

        let (x, mut c) = if s > 9 {
            (s%10, s/10)
        } else {
            (s, 0)
        };

        if k < l {
            result[k] += x;
            if result[k] > 9 {
                c += result[k]/10;
                result[k] %= 10;
            }
        }

        if k + 1 < l {
            result[k + 1] += c;
        } else if c > 0 {
            result.push(c);
        }

        i += 1;
        j += 1;
        k += 1;
    }

    result
}

fn maxu(a: usize, b: usize) -> usize {
    if a >= b {
        a
    } else {
        b
    }
}

fn maxi(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

pub fn max_length(arr: Vec<String>) -> i32 {
    // max of length without arr[0] and with arr[0]
    maxi(arr[0].len() as i32, max_length_helper((&arr, 0, 1, arr.len(), String::new())))
}

pub fn max_length_iter(arr: Vec<String>) -> i32 {
    let l: usize = arr.len().abs_diff(1);
    let mut m: i32 = 0;
    let mut stack: Vec<(usize, usize, String)> = Vec::with_capacity(l + 1);
    stack.push((0, 1, String::new()));
    while let Some((i, j, acc)) = stack.pop() {
        if i < l {
            if j < l {
                stack.push((i, j + 1, String::new() + &acc));
                stack.push((i, j + 1, String::new() + &acc + &*arr[j]));
            } else {
                m = maxi(
                    m,
                    maxi(
                        maxi(
                            count(&*arr[i], &acc),
                            count(&*arr[i], &*arr[j])
                        ),
                        maxi(
                            maxi(
                                count("", &acc),
                                count(&*arr[j], &acc)
                            ),
                            count(&*arr[i], &(acc + &*arr[j]))
                        )
                    )
                );
            }
        } else {
            m = maxi(m, arr[i].len() as i32);
        }
    }
    m
}

pub fn max_length_helper((arr, i, j, l, accum): (&Vec<String>, usize, usize, usize, String)) -> i32 {
    if i < l.abs_diff(1) {
        if j < l.abs_diff(1) {
            // max of length without j and length with j
            maxi(
                max_length_helper((arr, i, j + 1, l, String::new() + &accum)),
                max_length_helper((arr, i, j + 1, l, String::new() + &accum + &*arr[j]))
            )
        } else {
            // max of
            maxi(
                // max of arr[i] with accum and arr[i] with arr[j]
                maxi(
                    count(&*arr[i], &accum),
                    count(&*arr[i], &*arr[j])
                ),
                // and max of
                maxi(
                    // max of accum without arr[j] and accum with arr[j]
                    maxi(
                        count("", &accum),
                        count(&accum, &*arr[j])
                    ),
                    // and arr[i] with accum with arr[j]
                    count(&*arr[i], &(accum + &arr[j]))
                )
            )
        }
    } else {
        // i = l - 1 = 0
        arr[i].len() as i32
    }
}

fn count(a: &str, b: &str) -> i32 {
    let (x, y): (usize, usize) = (a.len(), b.len());
    let h: HashSet<char> = HashSet::from_iter((a.to_string() + b).chars());
    if h.len() == x + y {
        h.len() as i32
    } else {
        0
    }
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = long_add(Vec::from([9]), Vec::from([9]));
        assert_eq!(result, [8, 1]);
    }

    #[test]
    fn max_length_test() {
        // let result: i32 = max_length(Vec::from([
        //     "ta".to_string(),
        //     // "cdefghijklmnopqrstuv".to_string(),
        //     "tuv".to_string(),
        //     "au".to_string(),
        //     "be".to_string(),
        //     // "zxv".to_string(),
        // ]));
        let result: i32 = max_length_iter(Vec::from([
            "fc".to_string(),
            "abu".to_string(),
            "zx".to_string(),
            "cbd".to_string(),
            "efgu".to_string(),
            "ety".to_string(),
            "fe".to_string(),
        ]));
        // let result: i32 = max_length(Vec::from(["abu".to_string(), "cbd".to_string(), "d".to_string()]));
        // let result: i32 = max_length(Vec::from(["c".to_string(), "abu".to_string()]));
        // let result: i32 = max_length(Vec::from(["a".to_string(), "c".to_string(), "b".to_string(), "d".to_string(), "abc".to_string(), ]));
        // assert_eq!(result, 0);
        assert_eq!(result, 10);
    }
}
