pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (nums1, nums2) = if nums1.len() > nums2.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };

    let (m, n) = (nums1.len(), nums2.len());
    let (mut low, mut high) = (0, m);

    while low <= high {
        let partition_x = (low + high) / 2;
        let partition_y = (m + n).div_ceil(2) - partition_x;

        let max_x = nums1
            .get(partition_x.wrapping_sub(1))
            .copied()
            .unwrap_or(i32::MIN);

        let min_x = nums1.get(partition_x).copied().unwrap_or(i32::MAX);

        let max_y = nums2
            .get(partition_y.wrapping_sub(1))
            .copied()
            .unwrap_or(i32::MIN);
        let min_y = nums2.get(partition_y).copied().unwrap_or(i32::MAX);

        if max_x <= min_y && max_y <= min_x {
            if (m + n) % 2 == 0 {
                return (max_x.max(max_y) + min_x.min(min_y)) as f64 / 2.0;
            } else {
                return max_x.max(max_y) as f64;
            }
        } else if max_x > min_y {
            high = partition_x - 1;
        } else {
            low = partition_x + 1;
        }
    }

    0.0
}

fn main() {
    let result = find_median_sorted_arrays(vec![1, 3], vec![2]);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn example_2() {
        let result = find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.5);
    }
}

