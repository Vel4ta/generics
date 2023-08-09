use std::collections::HashSet;
// use std::slice::range;

fn long_add(v: Vec<usize>, w: Vec<usize>) -> Vec<usize> {
    let (mut i, mut j, mut k): (usize, usize, usize) = (0, 0, 0);
    let (vl, wl) = (v.len(), w.len());
    let mut done: bool = false;

    let l: usize = vl.max(wl);

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

pub fn gold_helper(grid: &Vec<Vec<i32>>, nodes: Vec<(usize, usize)>, current_node: (usize, usize), mut path: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if nodes.is_empty() {
        return path;
    }

    let mut potentials: Vec<(usize, usize)> = nodes.clone();
    potentials.retain(|&node|
        (current_node.0 == node.0 && (current_node.1 + 1 == node.1 || node.1 + 1 == current_node.1)) ||
        ((current_node.0 + 1 == node.0 || node.0 + 1 == current_node.0) && current_node.1 == node.1)
    );

    if !potentials.is_empty() {
        let mut max_paths: Vec<Vec<(usize, usize)>> = Vec::new();
        for node in potentials {
            let new_path: Vec<(usize, usize)> = Vec::from([node]);
            let mut copy_nodes = nodes.clone();
            copy_nodes.retain(|(a, b)| !(a == &node.0 && b == &node.1));
            max_paths.push(gold_helper(grid, copy_nodes, node, new_path));
        }

        if !max_paths.is_empty() {
            let mut max_gold = 0;
            let mut max_path_index = 0;
            for (index, p) in max_paths.clone().iter().enumerate() {
                let sum = p.iter()
                .map(|(i, j)| grid[*i][*j])
                .reduce(|sum, value|
                    sum + value
                ).unwrap();

                if sum > max_gold {
                    max_gold = sum;
                    max_path_index = index;
                }
            }
            path.append(&mut max_paths.remove(max_path_index));
        }
    }
    return path;
}


pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() {
        return  0
    }

    
    let col = grid.len();
    let (mut i, mut j) = (0, 0);
    let mut paths: Vec<(usize, usize)> = Vec::new();
    while i < col {
        while j < grid[i].len() {
            if grid[i][j] > 0 {
                paths.push((i, j));
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }


    let mut max_paths: Vec<Vec<(usize, usize)>> = Vec::new();
    let copy_path = paths.clone();
    for node in paths {
        let mut nodes = copy_path.clone();
        nodes.retain(|(a, b)| !(a == &node.0 && b == &node.1));
        max_paths.push(gold_helper(&grid, nodes, node, Vec::from([node])));
    }

    println!("{:?}", max_paths);
    let mut max_gold = 0;
    for path in max_paths {
        let mut sum = 0;
        for (i, j) in path {
            sum += grid[i][j];
        }
        if sum > max_gold {
            max_gold = sum;
        }
    }
    max_gold
}


pub fn longest_valid_parentheses(s: String) -> i32 {
    if s.is_empty() {
        return 0
    }

    let mut cleared = vec![];
    let mut stack = vec![];
    for (i, p) in s.char_indices() {
        cleared.push(false);
        if p == '(' {
            stack.push(i);
        } else if !stack.is_empty() {
            cleared[i] = true;
            cleared[stack.pop().unwrap()] = cleared[i];
        }
    }

    cleared.split(|item| *item == false).max().unwrap().len() as i32
    // 0
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn median_helper(mut nums: Vec<i32>, nums1: &[i32], nums2: &[i32], i: usize, j: usize, l1: usize, l2: usize, m: usize, n: usize) -> Vec<i32>{
    println!("{l1}, {i}, {l2}, {j}");
    if nums1[i] > nums2[j] {
        if nums1[l1] > nums2[j] {
            // nums = [&nums, &nums2[l2..j + 1], &nums1[l1..i + 1]].concat();
            if l2 == j {
                nums = [&nums, &nums1[l1..i + 1]].concat();
            } else {
                nums = [&nums, &nums2[l2..j + 1], &nums1[l1..i + 1]].concat();
            }
            return nums;
        }

        let mut temp_i = mid(i);
        let mut temp_j = mid(j);
        nums = median_helper(nums, nums1, nums2, temp_i, temp_j, l1, l2, m, n);
        temp_i += 1;
        temp_j += 1;
        if temp_i < i && temp_j < j {
            nums = median_helper(nums, nums1, nums2, i, j, temp_i, temp_j, m, n);
        }
    }

    if nums1[i] < nums2[j] {
        if nums1[i] <= nums2[l2] {
            nums = [&nums, &nums1[l1..i + 1], &nums2[l2..j + 1]].concat();
            // if l1 == i && l2 < j {
            //     nums = [&nums, &nums2[l2..j + 1]].concat();
            // } else if l1 + 1 <= m || l2 + 1 <= n {
            //     nums = [&nums, &nums1[l1..i + 1], &nums2[l2..j + 1]].concat();
            // }
            return nums;
        }

        let mut temp_i = mid(i);
        let mut temp_j = mid(j);
        nums = median_helper(nums, nums1, nums2, temp_i, temp_j, l1, l2, m, n);
        temp_i += 1;
        temp_j += 1;
        if temp_i < i && temp_j < j {
            nums = median_helper(nums, nums1, nums2, i, j, temp_i, temp_j, m, n);
        }
    }

    if nums1[i] == nums2[j] {
        if (l1 == i) & (l2 == j) {
            nums.push(nums1[i]);
            nums.push(nums2[j]);
            return nums
        } else if (l1 < i) & (l2 < j) {
            let mut temp_i = i;
            let mut temp_j = j;
            temp_i -= 1;
            temp_j -= 1;
            if temp_i > l1 && temp_j > l2 {
                nums = median_helper(nums, nums1, nums2, temp_i, temp_j, l1, l2, m, n);
            } else if temp_i == l1 && temp_j == l2 {

            } else if temp_i == l1  {
                nums = [&nums, &nums2[l2..temp_j + 1]].concat();
            } else {
                nums = [&nums, &nums1[l1..temp_i + 1]].concat();
            }

            nums.push(nums1[i]);
            nums.push(nums2[j]);
        } else if l1 == i {
            nums = [&nums, &nums2[l2..j + 1]].concat();
            return nums
        } else {
            nums = [&nums, &nums1[l1..i + 1]].concat();
            return nums
        }
    }
    
    nums
}

pub fn mid(x: usize) -> usize {
    x/2
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());
    let mut i = mid(m);
    let mut j = mid(n);
    let mut nums = vec![];
    
    if !nums1.is_empty() & !nums2.is_empty() {
        nums = median_helper(nums, &nums1[0..i+1], &nums2[0..j+1], i, j, 0, 0, m, n);
        println!("{:?}", nums);
        if i + 1 < m {
            i += 1;
        }
        if j + 1 < n {
            j += 1;
        }
        println!();
        nums = median_helper(nums, &nums1, &nums2, m - 1, n - 1, i, j, m, n);
        println!("{:?}", nums);

    } else if nums1.is_empty() & nums2.is_empty() {
        return 0.0;
    } else if nums1.is_empty() {
        nums = nums2;
    } else {
        nums = nums1;
    }

    
    

    let mid = mid(m + n - 1);
    if !(m + n).is_power_of_two() || mid == 0 {
        nums[mid] as f64
    } else {
        ((nums[mid] + nums[mid + 1]) as f64) / 2.0
    }
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
    fn median_test() {
        let cases = vec![
            (Vec::from([1,3]), Vec::from([2]), 2.0),
            (Vec::from([0,0]), Vec::from([0, 0]), 0.0),
            (Vec::from([1,3]), Vec::from([2, 7]), 2.5),
            (Vec::from([]), Vec::from([1]), 1.0),
            (Vec::from([0,0,0,0,0]), Vec::from([-1,0,0,0,0,0,1]), 0.0),
            (Vec::from([1,2]), Vec::from([3,4]), 2.5),
            (Vec::from([]), Vec::from([2, 3]), 2.5),
        ];
        for case in cases {
            println!("{:?}", case);
            assert_eq!(case.2, find_median_sorted_arrays(case.0, case.1));
        }
    }

    #[test]
    fn paren_test() {
        // let case = ")(".to_string();
        let mut cases = [")(", ")()())()()(", ")()())", ")(((((()())()()))()(()))(", "))))((()((", "()()))))()()(", "(()))())(", ")()))()(", ")()))(", "((())))))()", "())()(", ")(())))(())())", ")))))()())()", "))))())()()(()", "(()", "()()", "(()()", "()(()(((",
        "(()()(()(()))()((()))((()(()())()(()))())))()(()()))())))))))()()()()))(((()())((()()(((())))()(()()(())((()))))))(()(()))(((()())()))(()))((((()(()()()())()()(()))(()()(())()((()()())))(())()())()("];
        let results = cases.iter_mut().map(|s| longest_valid_parentheses(s.to_string()));
        let answers = [0, 4, 4, 22, 2, 4, 4, 2, 2, 6, 2, 6, 4, 4, 2, 4, 4, 2, 68];
        for result in results.enumerate() {
            assert_eq!(result.1, answers[result.0]);
        }
        // let result = longest_valid_parentheses(case);
        // assert_eq!(result,2);
        // assert_eq!(result,4);
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
