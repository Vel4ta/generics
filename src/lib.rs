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


// pub fn max_length(arr: Vec<String>) -> i32 {
//     let l = arr.len();
//     let mut results: Vec<i32> = vec![0; l * l];
//     results[0] = arr[0].len() as i32;

//     let (mut i, mut j): (usize, usize) = (0, 1);
//     while i < l {
//         while j <= l {
//             if i > j {
//                 results[i * l + j.abs_diff(1)] = maxi(results[j.abs_diff(1) * l + i], results[j.abs_diff(1) * l + i]);
//             } else if i == j {
//                 results[i * l + j.abs_diff(1)] = maxi(arr[j].len() as i32, results[j.abs_diff(1) * l + i]);
//             } else {
//                 results[i * l + j.abs_diff(1)] = maxi(results[i * l + j.abs_diff(1)], count(&*arr[i], &*arr[j.abs_diff(1)]));
//             }
//             j += 1;
//         }
//         j = 1;
//         i += 1;
//     }
//     results[l.abs_diff(1) * l.abs_diff(1) + l.abs_diff(1)]
// }

pub fn max_length(arr: Vec<String>) -> i32 {
    let l = arr.len();
    let mut results: Vec<i32> = vec![0; l * l];

    let mut m: i32 = 0;
    let (mut i, mut j): (usize, usize) = (0, 0);
    while i < l {
        let mut c: i32 = 0;
        while j < l {
            if i == j {
                c += arr[j].len() as i32;
                results[i * l + j] = arr[j].len() as i32;
            } else {
                let ct = count(&*arr[i], &*arr[j]);
                results[i * l + j] = ct;
                c += ct;
                if ct > 0 {
                    let mut done: bool = false;
                    let mut k: usize = j + 1;
                    let mut c2: i32 = 0;
                    while !done {
                        if k < l {
                            results[j * l + k] = count(&*arr[j], &*arr[k]);
                            results[i * l + k] = count(&*arr[i], &*arr[k]);

                            if results[j * l + k] > 0 && results[i * l + k] > 0 {
                                c2 += results[j * l + k];
                                j = k;
                                k += 1;
                            }  else {
                                m = maxi(m, c + c2);
                                done = true;
                            }
                        } else {
                            m = maxi(m, c + c2);
                            done = true;
                        }
                    }
                    
                    j = k;
                }
            }
            j += 1;
        }
        println!("{c}");
        m = maxi(m, c);
        i += 1;
        j = i;
    }
    println!("{:?}", results);
    m
}

fn count(a: &str, b: &str) -> i32 {
    let (x, y): (usize, usize) = (a.len(), b.len());
    let h: HashSet<char> = HashSet::from_iter((a.to_string() + b).chars());
    if h.len() == x + y {
        y as i32
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
        let result: i32 = max_length(Vec::from(["fc".to_string(), "cbd".to_string(), "zx".to_string(), "abu".to_string(), "e".to_string(), "efgu".to_string()]));
        assert_eq!(result, 0);
    }
}
