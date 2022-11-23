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

fn maxi(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

fn maxu(a: usize, b: usize) -> usize {
    if a >= b {
        a
    } else {
        b
    }
}

pub fn max_length_dp(arr: Vec<String>) -> i32 {
    let s: usize = arr.len();
    let mut l: Vec<usize> = vec![0; s];
    let mut prev: Vec<usize> = vec![0; s];
    l[0] = count("", &*arr[0]); 
    let mut i_1: usize = 0;
    for i in 1..s {
        let (mut m, mut k): (usize, usize) = (0, i);
        let mut local: Vec<usize> = vec![0; i];
        for j in 0..i {
            local[j] = count(&*arr[i], &*arr[j]);
            if local[j] > 0 {
                let (mut t, mut w): (usize, usize) = (j, local[j]);

                while prev[t] != t && local[prev[t]] > 0 {
                    w += local[prev[t]];
                    t = prev[t];
                }

                if m <= w {
                    m = w;
                    k = j
                }
            }
        }

        m += count("", &*arr[i]);

        if l[i_1] < m {
            l[i] = m;
        } else {
            l[i] = l[i_1];
        }

        prev[i] = k;

        i_1 = i;
    }

    // maxium length concatenation of strings with unique characters
    l[i_1] as i32
}

fn count(a: &str, b: &str) -> usize {
    let (x, y): (usize, usize) = (a.chars().count(), b.chars().count());
    let s: String = a.to_string() + b;
    let mut c =s.chars();
    let mut h: HashSet<char> = HashSet::with_capacity(x + y);
    while let Some(item) = c.next() {
        if !h.insert(item) {
            return 0
        }
    }
    // for item in s.chars() {
    //     if !h.insert(item) {
    //         return 0
    //     }
    // }
    return y
}


fn partioned_sums(row: &Vec<i32>, a: usize, b: usize, c: usize) -> (i32, i32) {
    let (mut sum1, mut sum2): (i32, i32) = (0, 0);
    // let (mut temp1, mut temp2): (usize, usize) = (j, b2);
    let mut temp: usize = a;

    while temp < b && row[temp] > 0 {
        sum1 += row[temp];
        temp += 1;
    }

    if row[temp] > 0 {
        sum1 += row[temp];
    }

    temp = b;
    while temp < c && row[temp] > 0 {
        sum2 += row[temp];
        temp += 1;
    }

    if row[temp] > 0 {
        sum2 += row[temp];
    }

    (sum1, sum2)
}

fn dist(a: usize, b: usize) -> usize {
    let mut c: usize = 0;
    let mut d: usize = a;
    while d < b {
        c += 1;
        d += 1;
    }
    return c
}

/*
In a gold mine grid of size m x n, each cell in this mine has an integer representing the amount of gold
in that cell, 0 if it is empty.

Return the maximum amount of gold you can collect under the conditions:

Every time you are located in a cell you will collect all the gold in that cell.
From your position, you can walk one step to the left, right, up, or down.
You can't visit the same cell more than once.
Never visit a cell with 0 gold.
You can start and stop collecting gold from any position in the grid that has some gold.
*/

pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() {
        return  0
    }

    let (mut i, mut j, mut k, mut i_1, mut j_1, mut k_1, mut w):
        (usize, usize, usize, usize, usize, usize, i32) = (0, 0, 0, 0, 0, 0, 0);
    let (m, n): (usize, usize) = (grid.len(), grid[j].len());
    //                        start pos     end pos     
    //                        (x1, y1)      (x2, y2)    amount of gold in the path
    let mut stack: Vec<Vec<(usize, usize, usize, usize, i32)>> = vec![Vec::new(); m];

    let mut max_gold: i32 = 0;

    while i < m {
        while j < n {
            k_1 = j_1;
            k = j;
            w = 0;
            while k < n && grid[i][k] > 0 {
                w += grid[i][k];
                k_1 = k;
                k += 1;
            }

            if max_gold < w {
                max_gold = w;
            }

            if w > 0 {
                println!("Current Path {:?}", (i, j, i, k_1, w));
                if i == i_1 {
                    stack[i].push((i, j, i, k_1, w));
                }
                if i_1 < i {
                    let (mut x1, mut y1, mut x2, mut y2, mut tot) = (i, j, i, k_1, w);
                    let stack_len: usize = stack[i_1].len();
                    let mut stack_index: usize = 0;
                    while stack_index < stack_len {
                        let (a1, b1, a2, b2, t) = stack[i_1][stack_index];
                        println!("comparing {:?}", (a1, b1, a2, b2, t));
                        stack_index += 1;

                        //   a    b
                        // c    d
                        if b1 < y1 && b2 < y2 {

                        } else if b1 < y1 && b2 == y2 {
                            
                        } else if b1 < y1 && b2 > y2 {
                            //    y1    y2
                            // b1          b2
                            let d: usize = dist(y1, y2);
                            if d == 0 || d^1 < d {
                                let mut left: i32 = 0;
                                let mut right: i32 = 0;
                                let mut si: usize = 0;
                                while si < i_1 {
                                    let sil = stack[si].len();
                                    let mut si_i: usize = 0;
                                    while si_i < sil {
                                        if stack[si][si_i].1 == b1 {
                                            left = stack[si][si_i].4;
                                        } else if stack[si][si_i].3 == b2 {
                                            right = stack[si][si_i].4;
                                        }
                                        si_i += 1;
                                    }
                                    si += 1;
                                }
                                left += partioned_sums(&grid[i_1], b1, b2, b2).0;
                                right += partioned_sums(&grid[i_1], y1, b2, b2).0;
                            } else {
                                tot += t;
                            }
                            println!("updated path {:?}", (x1, y1, x2, y2, tot));
                        } else if b1 == y1 && b2 == y2 {
                            x1 = a1;
                            y1 = b1;
                            tot += t;
                            println!("updated path {:?}", (x1, y1, x2, y2, tot));
                        } else if b1 == y1 && b2 > y2 {
                            let d: usize = dist(y1, y2);
                            if d == 0 || d^1 < d {
                                let right = partioned_sums(&grid[i_1], y1, b2, b2).0 - grid[i_1][y1];
                                if a1 < i_1 && tot - right - grid[i_1][y1] > right {
                                    x1 = a1;
                                    y1 = b1;
                                    tot = t - right + tot;
                                } else {
                                    x2 = a2;
                                    y2 = b2;
                                    tot = right + grid[i_1][y1] + tot;
                                }
                            } else {
                                x1 = a1;
                                y1 = b1;
                                tot += t;
                            }
                            println!("updated path {:?}", (x1, y1, x2, y2, tot));
                        } else if b1 == y1 && b2 < y2 {
                            x1 = a1;
                            y1 = b1;
                            tot += t;
                            println!("updated path {:?}", (x1, y1, x2, y2, tot));
                        } else if b1 > y1 && b2 < y2 {
                            let d: usize = dist(b1, b2);
                            if d == 0 || d^1 < d {
                                let left = partioned_sums(&grid[i], y1, b1, b1).0 - grid[i][b1];
                                let right = partioned_sums(&grid[i], b2, y2, y2).0 - grid[i][b2];
                                if left < right {
                                    // right
                                    //    b1   b2
                                    // j          k_1
                                    x1 = a1;
                                    y1 = b1;
                                    tot = tot - left + t;
                                } else {
                                    // left
                                    //   b1   b2
                                    // j         k_1
                                    x2 = a2;
                                    y2 = b2;
                                    tot = tot - right + t;
                                }
                            } else {
                                tot += t;
                            }
                            println!("updated path {:?}", (x1, y1, x2, y2, tot));
                        } else if b1 > y1 && b2 == y2 {
                            if a1 < i_1 {
                                x2 = a1;
                                y2 = b1;
                                tot += t;
                            } else {
                                x2 = a2;
                                y2 = b2;
                                tot += t;
                            }
                            println!("updated path {:?}", (x1, y1, x2, y2, tot));
                        } else if b1 > y1 && b2 > y2 {
                        } 
                    }

                    let path: (usize, usize, usize, usize, i32) = (x1, y1, x2, y2, tot);
                    if max_gold < path.4 {
                        max_gold = path.4;
                    }
                    println!("6inserting {:?}", path);
                    stack[i].push(path);
                }
            }

            if k == j {
                j_1 = j;
                j += 1;
            } else {
                j = k;
            }
        }

        println!("-------");
        i_1 = i;
        i += 1;
        j = 0;
    }

    max_gold
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
    fn gold_test() {
        // let result1 = get_maximum_gold(Vec::from([
        //     Vec::from([0,6,0]),
        //     Vec::from([5,8,7]),
        //     Vec::from([0,9,0])
        //     ]));
        let result = get_maximum_gold(
            [
                [1,0,7],
                [2,0,6],
                [3,4,5],
                [0,3,0],
                [9,0,20]
            ].iter_mut().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>());
        // let result = get_maximum_gold([
        //     [1,0,7,0,0,0],
        //     [2,0,6,0,1,0],
        //     [3,5,6,7,4,2],
        //     [4,3,1,0,2,0],
        //     [3,0,5,0,20,0]
        // ].iter_mut().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>());
        // assert_eq!(result, 60);
        // assert_eq!(result1, 24);
        assert_eq!(result, 218);
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
        // let result: i32 = max_length_dp(Vec::from([
        //     "fc".to_string(),
        //     "abu".to_string(),
        //     "zx".to_string(),
        //     "cbd".to_string(),
        //     "efgu".to_string(),
        //     "ety".to_string(),
        //     "c".to_string(),
        // ]));
        // let result: i32 = max_length_dp(Vec::from(["a".to_string(), "b".to_string(), "d".to_string(), "e".to_string(),"abd".to_string()]));
        // let result: i32 = max_length_dp(Vec::from(["c".to_string(), "abu".to_string()]));
        // let result: i32 = max_length_dp(Vec::from(["un".to_string(), "iq".to_string(), "ue".to_string()]));
        // let result: i32 = max_length_dp(Vec::from(["s".to_string(), "sa".to_string(), "m".to_string(), "e".to_string(), "mu".to_string(), "ei".to_string(),]));
        let result: i32 = max_length_dp(["s","sa","m","e","mu","ei"].iter().map(|x| x.to_string()).collect::<Vec<String>>());
        // let result: i32 = max_length_dp(["un","iq","ue"].iter().map(|x| x.to_string()).collect::<Vec<String>>());
        // let result: i32 = max_length_dp(["a","b","dx","ey","am","bn","abc","def"].iter().map(|x| x.to_string()).collect::<Vec<String>>());
        // let result: i32 = max_length_dp(["abc","def","bp","dq","eg","fh"].iter().map(|x| x.to_string()).collect::<Vec<String>>());
        // let result: i32 = max_length_dp(["abcdefghijklm","bcdefghijklmn","cdefghijklmno","defghijklmnop","efghijklmnopq","fghijklmnopqr","ghijklmnopqrs","hijklmnopqrst","ijklmnopqrstu","jklmnopqrstuv","klmnopqrstuvw","lmnopqrstuvwx","mnopqrstuvwxy","nopqrstuvwxyz","opqrstuvwxyza","pqrstuvwxyzab"].iter().map(|x| x.to_string()).collect::<Vec<String>>());
        // let result: i32 = max_length_dp(Vec::from(["c".to_string()]));
        // let result: i32 = max_length_dp(Vec::from(["a".to_string(), "abc".to_string(), "d".to_string(), "de".to_string(), "def".to_string()]));
        // let result: i32 = max_length_iter(Vec::from(["a".to_string(), "c".to_string(), "b".to_string(), "d".to_string(), "abc".to_string(), ]));
        assert_eq!(result, 6);
        // assert_eq!(result, 10);
    }
}
