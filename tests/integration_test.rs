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

    #[test]
    fn fs_aps_from_matrix_test() {
        let data_sample: Vec<Vec<i8>> = vec![
            vec![0, 1], 
            vec![0, 2]
        ];
        let expect_fs = vec![
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(1)
                },
                heuristic: None,
                value: Some(1)
             },
             Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(0)
                },
                heuristic: None,
                value: Some(0)
             },
             Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(0)
                },
                heuristic: None,
                value: Some(0)
             },
             Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(1)
                },
                heuristic: None,
                value: Some(2)
             },
             Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(1)
                },
                heuristic: None,
                value: Some(2)
             },
             Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(0)
                },
                heuristic: None,
                value: Some(0)
             },
             Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(0)
                },
                heuristic: None,
                value: Some(0)
             },
             Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(1)
                },
                heuristic: None,
                value: Some(1)
             }
            
        ];
        let expect_aps = vec![0, 2, 4, 6, 8];
        assert_eq!(fs_aps_from_matrix(data_sample).unwrap(), (expect_fs, expect_aps));
    }

}