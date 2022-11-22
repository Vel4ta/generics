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
                    let stack_len: usize = stack[i_1].len();
                    let mut stack_index: usize = 0;
                    let mut included: bool = false;
                    while stack_index < stack_len {
                        let (a1, b1, a2, b2, t) = stack[i_1][stack_index];
                        println!("comparing {:?}", (a1, b1, a2, b2, t));
                        stack_index += 1;
                        let path: (usize, usize, usize, usize, i32);


                        //   a    b
                        // c    d
                        if b1 < j && b2 < k_1 {

                        } else if b1 < j && b2 == k_1 {
                            
                        } else if b1 < j && b2 > k_1 {

                        } else if b1 == j && b2 == k_1 {
                            path = (a1, b1, i, k_1, w + t);

                            if max_gold < path.4 {
                                max_gold = path.4;
                            }
                            println!("inserting {:?}", path);
                            stack[i].push(path);

                        } else if b1 == j && b2 > k_1 {
                            let d: usize = dist(j, k_1);
                            path = if d == 0 || d^1 < d {
                                let right = partioned_sums(&grid[i_1], j, b2, b2).0 - grid[i_1][j];
                                if a1 < i_1 && t - right - grid[i_1][j] > right {
                                    (a1, b1, i, k_1, t - right + w)
                                } else {
                                    (i, j, a2, b2, right + grid[i_1][j] + w)
                                }
                            } else {
                                (a1, b1, i, k_1, w + t)
                            };

                            if max_gold < path.4 {
                                max_gold = path.4;
                            }
                            println!("inserting {:?}", path);
                            stack[i].push(path);
                        } else if b1 == j && b2 < k_1 {
                            let path = (a1, b1, i, k_1, w + t);
                            if max_gold < path.4 {
                                max_gold = path.4;
                            }
                            println!("inserting {:?}", path);
                            stack[i].push(path);
                        } else if b1 > j && b2 < k_1 {
                            let d: usize = dist(b1, b2);
                            path = if d == 0 || d^1 < d {
                                let left = partioned_sums(&grid[i], j, b1, b1).0 - grid[i][b1];
                                let right = partioned_sums(&grid[i], b2, k_1, k_1).0 - grid[i][b2];
                                if left < right {
                                    // right
                                    //    b1   b2
                                    // j          k_1
                                    (a1, b1, i, k_1, w - left + t)
                                } else {
                                    // left
                                    //   b1   b2
                                    // j         k_1
                                    (i, j, a2, b2, w - right + t)
                                }
                            } else {
                                (i, j, i, k_1, w + t)
                            };

                            if max_gold < path.4 {
                                max_gold = path.4;
                            }
                            println!("inserting {:?}", path);
                            stack[i].push(path);
                        } else if b1 > j && b2 == k_1 {
                            let path = 
                            if a1 < i_1 {
                                (i, j, a1, b1, w + t)
                            } else {
                                (i, j, a2, b2, w + t)
                            };
                            if max_gold < path.4 {
                                max_gold = path.4;
                            }
                            println!("inserting {:?}", path);
                            stack[i].push(path);
                        } else if b1 > j && b2 > k_1 {
                        } 
                    }
                }
            }

            if k == j {
                j_1 = j;
                j += 1;
            } else {
                j = k;
            }
        }

        let stack_len: usize = stack[i].len();
        let mut stack_index: usize = 0;

        while stack_index < stack_len {
            let (a1, b1, a2, b2, w) = stack[i][stack_index];
            let mut si = stack_index + 1;
            while si < stack_len {
                let (a3, b3, a4, b4, t) = stack[i][si];
                if a2 == a3 && b3 == b1 && b4 == b2 {
                    let (sum, _) = partioned_sums(&grid[a2], b3, b4, b4);
                    let path = (a1, b1, a4, b4, w + t - sum);
                    if max_gold < path.4 {
                        max_gold = path.4;
                    }
                    println!("found {:?}", path);
                    // stack[i].push(path);
                }
                si += 1;
            }
            stack_index += 1;
        }

        println!("-------");
        i_1 = i;
        i += 1;
        j = 0;
    }
    //                 while stack_index < stack_len {
    //                     let (a1, b1, a2, b2, t) = stack[i_1][stack_index];
    //                     println!("comparing {:?}", (a1, b1, a2, b2, t));
    //                     stack_index += 1;


    //                     if b1 >= j && b2 <= k_1 {
    //                         // let path: (usize, usize, usize, usize, i32);
    //                         if a1 == a2 && b1 == b2 {
    //                             let (sum1, sum2): (i32, i32) = partioned_sums(&grid[i], j, b2, k_1);
    //                             let total: i32 = sum1 + sum2 - grid[i][b2];
    //                             let path:
    //                                 (usize, usize, usize, usize, i32) =
    //                                 if t + sum2 >= total || sum1 < sum2 {
    //                                 // if sum1 < sum2 {
    //                                     (a1, b1, i, k_1, t + sum2)
    //                                 } else if t + sum1 >= total || sum1 >= sum2 {
    //                                 // } else if sum1 >= sum2 {
    //                                     (i, j, a1, b1, t + sum1)
    //                                 } else {
    //                                     (a1, b1, i, k_1, w + t)
    //                                 };
    //                             println!("inserting& {:?}", path);
    //                             if max_gold < path.4 {
    //                                 max_gold = path.4;
    //                             }
    //                             stack[i].push(path);
    //                             included = true;
    //                         } else if a2 == i_1 && b1 == j && grid[i_1][j] > 0 {
    //                             println!("inserting {:?}", (a1, b1, i, k_1, w + t));
    //                             if max_gold < w + t {
    //                                 max_gold = w + t;
    //                             }
    //                             stack[i].push((a1, b1, i, k_1, w + t));
    //                             included = true;
    //                         } else if a2 == i_1 && b1 > j && b1 == b2 {
    //                             let sums: (i32, i32) = partioned_sums(&grid[i], j, b2, k_1);
    //                             let path: (usize, usize, usize, usize, i32) = 
    //                                 if w + sums.0 >= t {
    //                                     (i, j, a1, b1, t + sums.0)
    //                                 } else {
    //                                     (a1, b1, i, k_1, t + sums.1)
    //                                 };
    //                             println!("inserting {:?}", path);
    //                             if max_gold < path.4 {
    //                                 max_gold = path.4;
    //                             }
    //                             stack[i].push(path);
    //                             included = true;
    //                         } else if a1 < i_1 {
    //                             let sums: (i32, i32) = partioned_sums(&grid[i_1], j, b2, b2);
    //                             let (sum1, sum2): (i32, i32) = (t - sums.0, sums.0);
    //                             let path:
    //                                 (usize, usize, usize, usize, i32) =
    //                                 if sum1 >= sum2 {
    //                                     (a1, b1, i, k_1, w + sum1 + grid[i_1][j])
    //                                 } else {
    //                                     if a1 < i_1 {
    //                                         (i, j, a1, b1, w + sum2)
    //                                     } else {
    //                                         (i, j, a2, b2, w + sum2)
    //                                     }
    //                                 };
    //                             println!("{sum1} {sum2}");
    //                             println!("inserting {:?}", path);
    //                             if max_gold < path.4 {
    //                                 max_gold = path.4;
    //                             }
    //                             stack[i].push(path);
    //                             included = true;
    //                         } else if a1 == i_1 {
    //                             let path:
    //                                 (usize, usize, usize, usize, i32) =
    //                                     (i, j, a2, b2, w + t);
                                
    //                             println!("inserting* {:?}", path);
    //                             if max_gold < path.4 {
    //                                 max_gold = path.4;
    //                             }
    //                             stack[i].push(path);
    //                         }
    //                         println!("what else");
    //                     } else if j >= b1 && k_1 <= b2 {
    //                         if j == k_1 {
    //                             let (sum1, sum2): (i32, i32);
    //                             let sums: (i32, i32);

    //                             if a1 < i_1 && grid[a2][b1] > 0 {
    //                                 sums = partioned_sums(&grid[i_1], k_1, b2, b2);
    //                                 sum1 = t - sums.0;
    //                                 sum2 = sums.0 - grid[i_1][k_1];
    //                                 // sum2 = sums.0;
    //                             } else if a2 < i_1 && grid[a1][j] > 0 {
    //                                 println!("do I need to split?");
    //                                 if b1 < j {
    //                                     sums = partioned_sums(&grid[i_1], b1, k_1, b2);
    //                                     sum2 = t - sums.0;
    //                                     sum1 = sums.0 - grid[i_1][k_1];
    //                                     // sum1 = sums.0;
    //                                 } else {
    //                                     sum1 = 0;
    //                                     sum2 = t - grid[i_1][k_1];
    //                                     // sum2 = t;
    //                                 }
    //                             } else {
    //                                 sums = partioned_sums(&grid[a1], b1, k_1, b2);
    //                                 sum1 = sums.0 - grid[i_1][k_1];
    //                                 // sum1 = sums.0;
    //                                 sum2 = sums.1 - grid[i_1][k_1];
    //                                 // sum2 = sums.1;
    //                             }
    //                             let total: i32 = sum1 + sum2;
    //                             let path:
    //                                 (usize, usize, usize, usize, i32) =
    //                                 // if sum1 < sum2 {
    //                                 //     // (i, j, a2, b2, w + sum2 + grid[i_1][k_1])
    //                                 //     (i, j, a2, b2, w + sum2)
    //                                 // } else {
    //                                 //     // (a1, b1, i, j, w + sum1 + grid[i_1][k_1])
    //                                 //     (a1, b1, i, j, w + sum1)
    //                                 // };
    //                                 if w + sum2 > total || sum1 < sum2 {
    //                                 // if sum1 < sum2 {
    //                                     (i, j, a2, b2, w + sum2 + grid[i_1][k_1])
    //                                 } else if w + sum1 >= total || sum1 >= sum2 {
    //                                 // } else if sum1 >= sum2 {
    //                                     (a1, b1, i, j, w + sum1 + grid[i_1][k_1])
    //                                 } else {
    //                                     (a1, b1, i, k_1, w + t)
    //                                 };

    //                             println!("{sum1} {sum2}");
    //                             println!("inserting {:?}", path);
    //                             if max_gold < path.4 {
    //                                 max_gold = path.4;
    //                             }
    //                             stack[i].push(path);
    //                             included = true;
    //                         // } else if a1 <= i_1 {
    //                         } else if a1 < i_1 {
    //                             let sums: (i32, i32) = partioned_sums(&grid[i_1], j, b2, b2);
    //                             let (sum1, sum2): (i32, i32) = (t - sums.0, sums.0);
    //                             let path:
    //                                 (usize, usize, usize, usize, i32) =
    //                                 if sum1 >= sum2 {
    //                                     (a1, b1, i, k_1, w + sum1 + grid[i_1][j])
    //                                 } else {
    //                                     (i, j, a2, b2, w + sum2)
    //                                 };
    //                             println!("{sum1} {sum2}");
    //                             println!("inserting {:?}", path);
    //                             if max_gold < path.4 {
    //                                 max_gold = path.4;
    //                             }
    //                             stack[i].push(path);
    //                             included = true;
    //                         } else if a1 == i_1 {
    //                             let path:
    //                                 (usize, usize, usize, usize, i32) =
    //                                     (i, j, a2, b2, w + t);
                                
    //                             println!("inserting* {:?}", path);
    //                             if max_gold < path.4 {
    //                                 max_gold = path.4;
    //                             }
    //                             stack[i].push(path);
    //                         }
    //                     }
    //                 }

    //                 if !included {
    //                     println!("inserting {:?}", (i, j, i, k_1, w));
    //                     stack[i].push((i, j, i, k_1, w));
    //                 }
    //             }
    //         }
    //         if k == j {
    //             j_1 = j;
    //             j += 1;
    //         } else {
    //             j = k;
    //         }
    //     }

    //     // let mut sl = stack[i].len();
    //     // let mut stack_index = 0;
    //     // while stack_index < sl {
    //     //     let (a1, b1, a2, b2, t) = stack[i][stack_index];
    //     //     println!("current path: {:?}", (a1, b1, a2, b2, t));
    //     //     // if max_gold < t {
    //     //     //     max_gold = t;
    //     //     // }
    //     //     for index in (stack_index + 1)..sl {
    //     //         let (a3, b3, a4, b4, w): (usize, usize, usize, usize, i32) = stack[i][index];
    //     //         println!("comparing: {:?}", (a3, b3, a4, b4, w));
    //     //         if a1 == a4 && b2 == b4 && a3 == i_1 && b3 == b1 && a2 < a3 {

    //     //             // check path parts from previous rows in stack

    //     //             let (sum2, _): (i32, i32) = partioned_sums(&grid[a1], b2, b4, b4);
    //     //             let sum1: i32 = t - sum2;
    //     //             println!("{sum1} {sum2}");
    //     //             let path = (a3, b3, a4, b4, w + sum1);
    //     //             println!("inserting***: {:?}", path);
    //     //             if max_gold < w + sum1 {
    //     //                 max_gold = w + sum1;
    //     //             }
    //     //             // stack[i].push(path);
    //     //             // sl += 1;
    //     //         } else
    //     //         if b3 == b1 && b2 == b4 && a2 == a3 && a4 == a1 {
    //     //             let (sum1, _) = partioned_sums(&grid[a3], b3, b2, b2);
    //     //             let total = w + t - sum1;
    //     //             if max_gold < total {
    //     //                 max_gold = total;
    //     //             }
    //     //             let path = (a1, b1, a4, b4, total);
    //     //             println!("inserting**: {:?}", path);
    //     //             stack[i].push(path);
    //     //             // sl += 1;
    //     //         }
    //     //     }
    //     //     stack_index += 1;
    //     // }

    //     println!("-------");
    //     i_1 = i;
    //     i += 1;
    //     j = 0;
    // }

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
        // let result = get_maximum_gold(
        //     [
        //         [1,0,7],
        //         [2,0,6],
        //         [3,4,5],
        //         [0,3,0],
        //         [9,0,20]
        //     ].iter_mut().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>());
        let result = get_maximum_gold([
            [1,0,7,0,0,0],
            [2,0,6,0,1,0],
            [3,5,6,7,4,2],
            [4,3,1,0,2,0],
            [3,0,5,0,20,0]
        ].iter_mut().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>());
        assert_eq!(result, 60);
        // assert_eq!(result1, 24);
        // assert_eq!(result, 28);
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
