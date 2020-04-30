#[cfg(test)]
mod tests {
    use esgi_arena_resolver_algorithms::algorithms::a_star::*;

    #[test]
    fn get_manhattan_distance_heuristic_test() {
        let start = Point { x: Some(0), y: Some(0) };
        let end = Point { x: Some(8), y: Some(8) };
        let data_sample = get_manhattan_distance_heuristic(start, end);
        assert_eq!(data_sample, 16);
    }

}