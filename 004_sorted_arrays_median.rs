impl Solution {
    /// nums1 and nums2 are sorted, and cannot both be empty
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let lists = [&nums1[..], &nums2[..]];
        let lists = &lists[..];
        let mut first_median = None;
        for list in lists.iter() {
            if list.len() == 0 { continue; }

            // Do a binary search for the median; we know if the result is low or high
            let mut left = 0;
            let mut right = list.len()-1;
            while left <= right {
                // Don't check for something we've already confirmed to be a median,
                // or else it will return it as *the* median
                let middle = (left + right) / 2;
                let guess = list[middle];

                if Some(guess) == first_median {
                    continue;
                }

                let res = is_a_median(guess, lists);
                match (first_median, res) {
                    (_, IsMedian::No(Dir::Lower)) => {
                        if middle == 0 {
                            // We don't want any underflows
                            break;
                        }
                        right = middle - 1;
                    }
                    (_, IsMedian::No(Dir::Higher)) => {
                        left = middle + 1;
                    }
                    (None, IsMedian::With(Dir::Lower)) => {
                        first_median = Some(guess);
                        if middle == 0 {
                            // We don't want any underflows
                            break;
                        }
                        right = middle - 1;
                    }
                    (None, IsMedian::With(Dir::Higher)) => {
                        first_median = Some(guess);
                        left = middle + 1;
                    }
                    (Some(first_median), IsMedian::With(_)) => return (guess as f64 + first_median as f64) / 2.0,
                    (Some(_), IsMedian::Yes) => panic!("Found definite median after indefinite median!"),
                    (None, IsMedian::Yes) => return guess as f64,
                }
            }
        }
        panic!("No median found!");
    }
}

#[derive(Debug)]
enum Dir {
    Lower,
    Higher
}

#[derive(Debug)]
enum IsMedian {
    Yes,
    /// This is *a* median, but you need one more in the specified direction
    With(Dir),
    /// This is not a median, and you can find one in the specified direction
    No(Dir),
}

fn is_a_median(guess: i32, lists: &[&[i32]]) -> IsMedian {
    let mut left = 0;
    let mut right = 0;
    let mut total_size = 0;
    for list in lists {
        total_size += list.len();
        match binary_search_left(guess, list) {
            Ok(0) | Err(0) => {}
            Ok(r) | Err(r) => {
                left += r;
            }
        }
        match binary_search_right(guess, list) {
            Ok(r) => {
                right += list.len() - 1 - r;
            }
            Err(r) => {
                right += list.len() - r;
            }
        }
    }
    let size = total_size - left - right;
    if left == right || (right > left && right - left < size) || (left > right && left - right < size) {
        IsMedian::Yes
    } else if (right > left && right - left == size) {
        IsMedian::With(Dir::Higher)
    } else if (left > right && left - right == size) {
        IsMedian::With(Dir::Lower)
    } else if right > left {
        IsMedian::No(Dir::Higher)
    } else {
        IsMedian::No(Dir::Lower)
    }
}

/// A binary search that returns the left hand boundary and right hand boundary
/// of elements. Left and right will be the same, *unless* there are duplicates
/// of the element you are trying to find.
///
/// https://stackoverflow.com/questions/13197552/using-binary-search-with-sorted-array-with-duplicates
fn binary_search_left(val: i32, nums: &[i32]) -> Result<usize, usize> {
    let left_hook = |val: i32, nums: &[i32], idx: usize| false;
    let right_hook = |val: i32, nums: &[i32], idx: usize| idx != 0 && nums[idx-1] == val;
    binary_search(val, nums, left_hook, right_hook)
}

fn binary_search_right(val: i32, nums: &[i32]) -> Result<usize, usize> {
    let left_hook = |val: i32, nums: &[i32], idx: usize| idx < nums.len()-1 && nums[idx+1] == val;
    let right_hook = |val: i32, nums: &[i32], idx: usize| false;
    binary_search(val, nums, left_hook, right_hook)
}

fn binary_search<L, R>(val: i32, nums: &[i32], hook_left: L, hook_right: R) -> Result<usize, usize>
where
    L: Fn(i32, &[i32], usize) -> bool,
    R: Fn(i32, &[i32], usize) -> bool,
{
    if nums.len() == 0 {
        return Err(0);
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let middle = (left + right) / 2;
        if nums[middle] < val || hook_left(val, nums, middle) {
            left = middle + 1;
        } else if nums[middle] > val || hook_right(val, nums, middle){
            if middle == 0 {
                // We don't want any underflows
                break;
            }
            right = middle - 1;
        } else {
            return Ok(middle);
        }
    }
    // At this point the "left" bounds index is to the right of the "right" bounds index.
    // We return the more right "left" because that is the place where you would insert.
    Err(left)
}
