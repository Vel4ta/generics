use std::{collections::HashSet, ops::Range};
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


pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let l = s.len();
    if l < 2 {
        return l as i32;
    }

    let (mut max_len, mut bag_start, mut bag_end, mut count) = (0, 0, 1, 0);
    let mut i;
    while bag_end < l {
        i = bag_start;
        while i < bag_end {
            if s[i] == s[bag_end] {
                bag_start = i + 1;
                break;
            }
            i += 1;
            count += 1;
        }

        count += 1;

        if count > max_len {
            max_len = count;
        }

        count = 0;
        bag_end += 1;
    }
    

    max_len as i32
}

pub fn tree_height(tree: &[(usize, usize, usize, usize, usize, usize)], node_index: usize, child_index: usize) -> usize {
    if node_index == child_index {
        return 0
    }

    return tree[child_index].5
}

pub fn left_rotate(tree: &mut[(usize, usize, usize, usize, usize, usize)], index: usize) -> usize {
    // println!("before left {index} {:?}", tree);
    let child = tree[index].3;
    let grandchild_l = tree[child].2;

    tree[index].3 = if grandchild_l == child {
        index
    } else {
        tree[grandchild_l].0 = index;
        grandchild_l
    };

    let parent = tree[index].0;
    tree[index].0 = child;
    if parent != index {
        if tree[parent].2 == index {
            tree[parent].2 = child;
        } else {
            tree[parent].3 = child;
        }
    }

    tree[child].0 = if parent == index {
        child
    } else  {
        parent
    };
    tree[child].2 = index;

    tree[index].5 = tree_height(tree, index, tree[index].2).max(tree_height(tree, index, tree[index].3)) + 1;
    tree[child].5 = tree_height(tree, child, tree[child].2).max(tree_height(tree, child, tree[child].3)) + 1;

    // println!("after left {index} {:?}", tree);
    return child;
}

pub fn right_rotate(tree: &mut[(usize, usize, usize, usize, usize, usize)], index: usize) -> usize {
    // println!("before right {index} {:?}", tree);
    let child = tree[index].2;
    let grandchild_r = tree[child].3;

    tree[index].2 = if grandchild_r == child {
        index
    } else {
        tree[grandchild_r].0 = index;
        grandchild_r
    };

    let parent = tree[index].0;
    tree[index].0 = child;

    if parent != index {
        if tree[parent].2 == index {
            tree[parent].2 = child;
        } else {
            tree[parent].3 = child;
        }
    }

    tree[child].0 = if parent == index {
        child
    } else  {
        parent
    };
    tree[child].3 = index;

    tree[index].5 = tree_height(tree, index, tree[index].2).max(tree_height(tree, index, tree[index].3)) + 1;
    tree[child].5 = tree_height(tree, child, tree[child].2).max(tree_height(tree, child, tree[child].3)) + 1;

    // println!("after right {index} {:?}", tree);
    return child;
}

pub fn tree_test(s: String) {
    println!("{s}");
    let s = s.as_bytes();
    let l = s.len();
    // if l < 2 {
    //     return l as i32;
    // }

    //                 parent, index, left, right, index in s, height
    let mut tree: Vec<(usize, usize, usize, usize, usize, usize)> = Vec::new();
    let (mut max, mut count, mut start, mut t_root) = (1, 1, 0, 0);
    let mut node;

    tree.push((start, start, start, start, start, 1));

    for i in 1..l {
        node = tree[t_root];
        while node.2 != node.3 {
            if s[i] < s[node.4] && node.2 != node.1 {
                node = tree[node.2]
            } else if s[i] > s[node.4] && node.3 != node.1 {
                node = tree[node.3];
            } else {
                break;
            }
        }

        let tl = tree.len();
        if s[i] < s[node.4] {
            tree[node.1].2 = tl;
            tree.push((node.1, tl, tl, tl, i, 1));
            count += 1;
        } else if s[i] > s[node.4] {
            tree[node.1].3 = tl;
            tree.push((node.1, tl , tl, tl, i, 1));
            count += 1;
        } else if i - node.4 == 1 {
            t_root = 0;
            start = i;
            count = 1;
            tree = [(t_root, t_root, t_root, t_root, start, 1)].to_vec();
        } else {
            count = if node.4 >= start {
                i - node.4
            } else {
                count + 1
            };
            tree[node.1].4 = i;
            if node.4 >= start {
                start = node.4 + 1;
            }
        }

        max = max.max(count);

        if !(tree.len() > tl) {
            println!("root:{t_root}, max:{max}, count:{count}, start:{start}, i:{i}, s[i]:{:?}\n {:?}", s[i], tree);
            continue;
        }

        node = tree[node.1];
        loop {
            let left = tree_height(&tree, node.1, node.2);
            let right = tree_height(&tree, node.1, node.3);
            tree[node.1].5 = 1 + left.max(right);

            let balance = left.max(right) - left.min(right);
            let sign = left < right;

            let new_root = if balance > 1 && !sign && s[i] < s[tree[node.2].4] {
                right_rotate(&mut tree, node.1)
            } else if balance > 1 && sign && s[i] > s[tree[node.3].4] {
                left_rotate(&mut tree, node.1)
            } else if balance > 1 && !sign && s[i] > s[tree[node.2].4] {
                tree[node.1].2 = left_rotate(&mut tree, node.2);
                right_rotate(&mut tree, node.1)
            } else if balance > 1 && sign && s[i] < s[tree[node.3].4] {
                tree[node.1].3 = right_rotate(&mut tree, node.3);
                left_rotate(&mut tree, node.1)
            } else {
                t_root
            };
            
            if node.0 == node.1 {
                t_root = new_root;
                break;
            }

            node = tree[node.1];
            node = tree[node.0];
        }

        println!("root:{t_root}, max:{max}, count:{count}, start:{start}, i:{i}, s[i]:{:?}\n {:?}", s[i], tree);
    }

    println!("root:{t_root}, max:{max}, count:{count} {:?}", tree);
    println!("{max}");
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}



pub fn mid(x: usize) -> usize {
    if x <= 2 {
        1
    } else {
        x/2 + 1
    }
}

pub fn mid_range(x: usize) -> Range<usize> {
    let m = mid(x);
    let m1 = m+1;
    if m > 0 {
        (m - 1*((x+1) & 1))..m1
    } else {
        m..m1
    }
}

// works but not optimal
pub fn median_splitter(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let (n, m) = (nums1.len(), nums2.len());
    let (mn, mm) = (mid(n), mid(m));
    if n == 0 || m == 0 || nums1[n - 1] <= nums2[0] {
        [nums1, nums2].concat()
    } else if nums1[0] >= nums2[m - 1] {
        [nums2, nums1].concat()
    } else if nums1[mn-1] <= nums2[0] {
        [&nums1[..mn], &median_splitter(&nums1[mn..], nums2)].concat()
    } else if nums1[0] >= nums2[mm-1] {
        [&nums2[..mm], &median_splitter(nums1, &nums2[mm..])].concat()
    } else {
        let left = median_splitter(&nums1[..mn], &nums2[..mm]);
        let right = median_splitter(&nums1[mn..], &nums2[mm..]);
        median_splitter(&left, &right)
    }
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let l = nums1.len() + nums2.len();
    let m = l/2;
    let r = if m > 0 {
        (m - 1*((l+1) & 1))..(m+1)
    } else {
        m..(m+1)
    };
    let l = r.len() as f64;
    median_splitter(&nums1, &nums2)[r].iter().sum::<i32>() as f64/l
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_test_tester() {
        let cases = vec![
            "abcabba",
            "pwwkew",
            "egcq",
            "ckilbkd",
            "abcabcbb",
            "ywjvusw",
            "asljlj",
            "vqblqcb",
            "asjrgapa",
            "gsqygebs",
            "wobgrovw",
            "eeydgwdykpv"
        ];
        for case in cases {
            tree_test(case.to_string());
        }
        assert_eq!(true, true);
    }

    #[test]
    fn mid_range_test() {
        let cases = vec![
            (0, 0..1),
            (1, 0..1),
            (2, 0..2),
            (3, 1..2),
            (4, 1..3),
            (5, 2..3),
            (6, 2..4),
        ];
        for (query,expected) in cases {
            println!("{:?}", query);
            let result = mid_range(query);
            println!("{:?}", result);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn mid_test() {
        let cases = vec![
            (0, 0),
            (1, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (5, 2),
            (6, 3),
        ];
        for (query,expected) in cases {
            println!("{:?}", query);
            assert_eq!(mid(query), expected);
        }
    }

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
            (Vec::from([3]), Vec::from([1, 2]), 2.0),
            (Vec::from([2]), Vec::from([1,3]), 2.0),
            (Vec::from([1,2,3,5]), Vec::from([4]), 3.0),
            (Vec::from([1,3,4,5]), Vec::from([2]), 3.0),
            (Vec::from([1,3,4,5,6]), Vec::from([2]), 3.5),
            (Vec::from([2,3,4,5,6]), Vec::from([1]), 3.5),
            (Vec::from([2,4,5,6]), Vec::from([1,3]), 3.5),
            (Vec::from([4]), Vec::from([1,2,3,5,6]), 3.5),
            (Vec::from([1,2,3,5]), Vec::from([4,6,7,8]), 4.5),
            (Vec::from([1,2,3,5]), Vec::from([4,6,7,8,9,10]), 5.5),
            (Vec::from([1,1,1,1,1,1,1,1,1,1,4,4]), Vec::from([1,3,4,4,4,4,4,4,4,4,4]), 3.0),

            (Vec::from([1,1,1,2,4]), Vec::from([1,2,3,5,6]), 2.0),

            (Vec::from([1,2,4]), Vec::from([1,2,4]), 2.0),

            (Vec::from([5]), Vec::from([1,2,3,4,6]), 3.5),

            (Vec::from([1,2,3,4,6]),Vec::from([5]), 3.5),
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
