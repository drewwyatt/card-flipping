static NO_SOLUTION: &str = "no solution";

pub fn find_solution(input: &str) -> &str {
    NO_SOLUTION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!("1 0 2 3 5 4 6", find_solution("0100110"))
    }

    #[test]
    fn sample_2() {
        assert_eq!(NO_SOLUTION, find_solution("01001100111"))
    }

    #[test]
    fn sample_3() {
        assert_eq!(
            "0 1 2 3 4 6 5 7 8 11 10 9 12 13 14",
            find_solution("100001100101000")
        )
    }
}
