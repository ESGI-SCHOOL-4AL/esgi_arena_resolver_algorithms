#[cfg(test)]
mod tests {
    use esgi_arena_resolver_algorithms::algorithms::a_star::*;

    #[test]
    fn get_manhattan_distance_heuristic_test() {
        let data_sample = get_manhattan_distance_heuristic([[0, 0], [8, 8]]);
        assert_eq!(data_sample, 16);
    }
}