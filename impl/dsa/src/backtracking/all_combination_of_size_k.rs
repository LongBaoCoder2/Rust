#[allow(dead_code)]
pub fn generate_all_combinations(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut current_state = vec![];

    try_the_next_state(&mut current_state, n, k, &mut result);

    result
}

#[allow(dead_code)]
pub fn try_the_next_state(
    current_state: &mut Vec<i32>,
    n: i32,
    k: i32,
    result: &mut Vec<Vec<i32>>,
) {
    if k == 0 {
        result.push(current_state.clone());
        return;
    }

    let last_state = if current_state.is_empty() {
        0
    } else {
        current_state[current_state.len() - 1]
    };

    for i in last_state + 1..n + 1 {
        current_state.push(i);
        try_the_next_state(current_state, n, k - 1, result);
        current_state.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_the_base_result() {
        let result = generate_all_combinations(0, 0);
        let expected_res: Vec<Vec<i32>> = vec![vec![]];

        assert_eq!(expected_res, result);
    }

    #[test]
    pub fn test_generate_all_combinations() {
        let expected_res = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        let result = generate_all_combinations(4, 2);
        assert_eq!(expected_res, result);
    }

    #[test]
    pub fn test_generate_all_combinations_complex() {
        let expected_res = vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]];
        let result = generate_all_combinations(4, 3);
        assert_eq!(expected_res, result);
    }

    #[test]
    pub fn test_generate_all_combinations_complex_6_3() {
        let expected_res = vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 2, 5],
            vec![1, 2, 6],
            vec![1, 3, 4],
            vec![1, 3, 5],
            vec![1, 3, 6],
            vec![1, 4, 5],
            vec![1, 4, 6],
            vec![1, 5, 6],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 3, 6],
            vec![2, 4, 5],
            vec![2, 4, 6],
            vec![2, 5, 6],
            vec![3, 4, 5],
            vec![3, 4, 6],
            vec![3, 5, 6],
            vec![4, 5, 6],
        ];
        let result = generate_all_combinations(6, 3);
        assert_eq!(expected_res, result);
    }
}
