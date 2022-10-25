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
    let l = arr.len();
    let mut results: Vec<i32> = Vec::with_capacity(l * l);
    results[0] = arr[0].len() as i32;

    let (mut i, mut j): (usize, usize) = (0, 1);
    while i < l {
        while j < l {
            if i > j {
                results[i * l + j] = maxi(results[j * l + i], results[(j - 1) * l + i]);
            } else if i == j {
                results[i * l + j] = maxi(results[i * l + j], results[i * l + (j - 1)]);
            } else {
                results[i * l + j] = maxi(results[i * l + (j - 1)], count(&*arr[i], &*arr[j]));
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    results[l * l - 1 + l]
}

fn count(a: &str, b: &str) -> i32 {
    let (mut i, mut j, x, y): (usize, usize, usize, usize) = (0, 0, a.len(), b.len());
    let l: usize = maxu(x, y);
    0

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
}
