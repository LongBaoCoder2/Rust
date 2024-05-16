#[allow(dead_code)]
pub fn permute(nums: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new(); // Vector to store the resulting permutations

    try_the_next_state(
        &mut vec![],
        &mut vec![false; nums.len()],
        &mut result,
        &nums,
    ); // Call the backtracking function
    result // Return the list of permutations
}

fn try_the_next_state(
    current_permutation: &mut Vec<i32>,
    is_used: &mut Vec<bool>,
    result: &mut Vec<Vec<i32>>,
    nums: &Vec<i32>,
) {
    if current_permutation.len() == nums.len() {
        result.push(current_permutation.clone());
        return;
    }

    for i in 0..nums.len() {
        if is_used[i] {
            continue;
        }

        is_used[i] = true;
        current_permutation.push(nums[i]);

        try_the_next_state(current_permutation, is_used, result, nums);

        is_used[i] = false;
        current_permutation.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        // Test case: Generate permutations for [1, 2, 3]
        let permutations = permute(&vec![1, 2, 3]);

        assert_eq!(permutations.len(), 6); // There should be 6 permutations

        // Verification for some of the permutations
        assert!(permutations.contains(&vec![1, 2, 3]));
        assert!(permutations.contains(&vec![1, 3, 2]));
        assert!(permutations.contains(&vec![2, 1, 3]));
    }
}
