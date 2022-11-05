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

pub fn max_length_dp(arr: Vec<String>) -> i32 {
    // size of arr
    let s: usize = arr.len();

    // l[k] = maximum length a concatenation of strings comprised from unique characters in the range 0..k can be
    // s.t. 1 <= k < s
    let mut l: Vec<usize> = vec![0; s];

    // prev[k] = index of previous string in the path up to k
    let mut prev: Vec<usize> = vec![0; s];

    // max length is the length of arr[0] when size is 1
    l[0] = arr[0].len();

    let mut cc = 0;
    // l[0] has been solved so solve for l[i] s.t. i = 1, 2, ..., s-1
    for i in 1..s {
        // m = maximum length at j s.t. j < i
        // k = index of string that produces m
        let (mut m, mut k): (usize, usize) = (0, i);

        // holds the maximum length possible of arr[i] + arr[j]
        // s.t. 0 <= j < i
        // so local[j] can be referenced later instead of redoing the comparison
        let mut local: Vec<usize> = vec![0; i];
        println!("-------->");
        for j in 0..i {
            cc += 1;
            println!("{i}, {j}, {cc}");
            if j > 0 {
                
                // store arr[i] + arr[j] for later
                local[j] = countu(&*arr[i], &*arr[j]);

                // if arr[i] + arr[i] is still unique
                if local[j] > 0 {

                    // verifies path to maximum length at k for i
                    let mut ok: bool = false;

                    // t = key to prev[t]
                    // w = sum of maximum length concatenations of unique strings in range 0..j for i
                    let (mut t, mut w): (usize, usize) = (j, local[j]);

                    while prev[t] != t {
                        cc += 1;
                        println!("^^^{i}, {}, {cc}", prev[t]);

                        // accum max length concatenation at prev[t]
                        w += local[prev[t]];

                        // if arr[i] + arr[prev[t]] is still unique
                        if local[prev[t]] > 0 {
                            // go to previous string
                            t = prev[t];
                            ok = true;
                        } else {
                            // no longer unique
                            m = maxu(m, w);
                            ok = false;
                            break;
                        }
                    }

                    // remained unique
                    if ok {
                        m = maxu(m, l[j]);
                    }

                    // if the maximum length concatenation at j for i is w or l[j]
                    if (m == w) || (m == l[j]) {
                        // update the index of the string that produces m
                        k = j;
                    }
                }
            } else {
                // prev[j] = j = 0

                // store length for later
                local[j] = countu(&*arr[i], &*arr[j]);

                // if arr[i] + arr[j] is still unique
                if local[j] > 0 {
                    // update the maximum length for i
                    m = local[j];
                    // update the index of the string that produces m
                    k = j;
                }
            }
        }
        println!("<---------");
        /*
            maximum length concatenation at i is
            the max of 
                the length of arr[i] + m = the length of arr[i] + maximum length concatention at k for i
                and the maximum length concatenation at i - 1
        */
        l[i] = maxu(arr[i].len() + m, l[i.abs_diff(1)]);

        // if the path to m at i is the maximum length concatenation at i
        if l[i] == arr[i].len() + m {
            // set prev[i] = index of string that produces m at i
            prev[i] = k;
        } else {
            // prev[i] = index of string that produces maximum concatenation at i
            prev[i] = i.abs_diff(1);
        }
    }

    // maxium length concatenation of strings with unique characters
    l[s.abs_diff(1)] as i32
}


// 2^(n - 1) : n > 4 -> not good
pub fn max_length_iter(arr: Vec<String>) -> i32 {
    let l: usize = arr.len().abs_diff(1);
    let mut m: i32 = 0;

    // bitwise operation to generate all permutations 2^n
    // no stack required
    // this is not a final solution to this problem
    // just a different approach to current implementation
    // while i < (1 << l) {
    //     print!("{}:[", i%l);
    //     while j < l {
    //         if (i & (1 << j)) > 0 {
    //             result[i%l * l + j] += count(&*arr[i%l], &*arr[j]);
    //             print!("({j}: {}) ", &*arr[j]);
    //         }
    //         j += 1;
    //     }
    //     i += 1;
    //     j = 0;
    //     println!("]");
    // }

    let mut stack: Vec<(usize, usize, String)> = Vec::with_capacity(l + 1);
    stack.push((0, 1, String::new()));
    let mut c: usize = 0;
    while let Some((i, j, acc)) = stack.pop() {
        // c += 1;
        // println!("{c}: {i}, {j}, {}", &acc);
        if i < l {
            if j < l {
                println!("{c}: {i}, {j}, {}, {m}", &acc);
                stack.push((i, j + 1, String::new() + &acc));
                if count(&*arr[j], &acc) > 0 {
                    stack.push((i, j + 1, String::new() + &acc + &*arr[j]));
                }
            } else {
                c += 1;
                println!("{c}: {i}, {j}, {}, {m}", &acc);
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
    println!("{c}");
    m
}

fn countu(a: &str, b: &str) -> usize {
    let (x, y): (usize, usize) = (a.len(), b.len());
    let s: String = a.to_string() + b;
    let mut c = s.chars();
    let mut h: HashSet<char> = HashSet::with_capacity(x + y);
    while let Some(item) = c.next() {
        if !h.insert(item) {
            return 0
        }
    }

    return y
}

fn count(a: &str, b: &str) -> i32 {
    let (x, y): (usize, usize) = (a.len(), b.len());
    let h: HashSet<char> = HashSet::from_iter((a.to_string() + b).chars());
    if h.len() == x + y {
        // h.len() as i32
        y as i32
    } else {
        // maxu(x, y) as i32
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
        // let result: i32 = max_length_dp(Vec::from([
        //     "ta".to_string(),
        //     "cdefghijklmnopqrstuv".to_string(),
        //     "tuv".to_string(),
        //     "au".to_string(),
        //     "be".to_string(),
        //     "zxv".to_string(),
        //     "c".to_string(),
        //     "d".to_string(),
        //     "e".to_string(),
        // ]));
        // let result: i32 = max_length_dp(Vec::from([
        //     "a".to_string(),
        //     "d".to_string(),
        //     "e".to_string(),
        //     "b".to_string(),
        //     "s".to_string(),
        //     "v".to_string(),
        //     "r".to_string(),
        //     "g".to_string(),
        // ]));
        let result: i32 = max_length_dp(Vec::from([
            "fc".to_string(),
            "abu".to_string(),
            "zx".to_string(),
            "cbd".to_string(),
            "efgu".to_string(),
            "ety".to_string(),
            "c".to_string(),
        ]));
        // let result: i32 = max_length_dp(Vec::from(["a".to_string(), "b".to_string(), "d".to_string(), "e".to_string(),"abd".to_string()]));
        // let result: i32 = max_length_dp(Vec::from(["c".to_string(), "abu".to_string()]));
        // let result: i32 = max_length_dp(Vec::from(["c".to_string(), "abu".to_string(), "da".to_string()]));
        // let result: i32 = max_length_dp(Vec::from(["c".to_string()]));
        // let result: i32 = max_length_iter(Vec::from(["a".to_string(), "c".to_string(), "b".to_string(), "d".to_string(), "abc".to_string(), ]));
        assert_eq!(result, 0);
        // assert_eq!(result, 10);
    }
}
