#[cfg(test)]
mod tests {
    use esgi_arena_resolver_algorithms::algorithms::a_star::*;

    fn testing_data() -> (Vec<Vec<i8>>, Vec<Field>, Vec<u8>) {
        let matrix_example: Vec<Vec<i8>> = vec![
            vec![0, 1], 
            vec![0, 2]
        ];

        let fs_example: Vec<Field> = vec![
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(1)
                },
                value: Some(1)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(0)
                },
                value: Some(0)
            },
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(0)
                },
                value: Some(0)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(1)
                },
                value: Some(2)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(1)
                },
                value: Some(2)
            },
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(0)
                },
                value: Some(0)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(0)
                },
                value: Some(0)
            },
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(1)
                },
                value: Some(1)
            }
            
        ];

        let aps_example: Vec<u8> = vec![0, 2, 4, 6, 8];

        return (matrix_example, fs_example, aps_example);
    }

    fn testing_data_heavy_matrix() -> (Vec<Vec<i8>>, Vec<Field>, Vec<u8>) {
        let matrix_example: Vec<Vec<i8>> = vec![
            vec![2, 0, 0], 
            vec![-1, -1, 0],
            vec![1, 0, 0]
        ];

        let matrix_first_line = vec![Field {
            coordinates: Point {
                x: Some(0),
                y: Some(1)
            },
            value: Some(0)
        },
        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(0)
            },
            value: Some(-1)
        },

        Field {
            coordinates: Point {
                x: Some(0),
                y: Some(0)
            },
            value: Some(2)
        },
        Field {
            coordinates: Point {
                x: Some(0),
                y: Some(2)
            },
            value: Some(0)
        },
        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(1)
            },
            value: Some(-1)
        },

        Field {
            coordinates: Point {
                x: Some(0),
                y: Some(1)
            },
            value: Some(0)
        },
        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(2)
            },
            value: Some(0)
        }];

        let matrix_second_line = vec![Field {
            coordinates: Point {
                x: Some(0),
                y: Some(0)
            },
            value: Some(2)
        },
        Field {
            coordinates: Point {
                x: Some(2),
                y: Some(0)
            },
            value: Some(1)
        },
        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(1)
            },
            value: Some(-1)
        },

        Field {
            coordinates: Point {
                x: Some(0),
                y: Some(1)
            },
            value: Some(0)
        },
        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(0)
            },
            value: Some(-1)
        },
        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(2)
            },
            value: Some(0)
        },
        Field {
            coordinates: Point {
                x: Some(2),
                y: Some(1)
            },
            value: Some(0)
        },

        Field {
            coordinates: Point {
                x: Some(0),
                y: Some(2)
            },
            value: Some(0)
        },
        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(1)
            },
            value: Some(-1)
        },
        Field {
            coordinates: Point {
                x: Some(2),
                y: Some(2)
            },
            value: Some(0)
        }];

        let matrix_third_line = vec![Field {
            coordinates: Point {
                x: Some(1),
                y: Some(0)
            },
            value: Some(-1)
        },
        Field {
            coordinates: Point {
                x: Some(2),
                y: Some(1)
            },
            value: Some(0)
        },

        Field {
            coordinates: Point {
                x: Some(2),
                y: Some(0)
            },
            value: Some(1)
        },
        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(1)
            },
            value: Some(-1)
        },
        Field {
            coordinates: Point {
                x: Some(2),
                y: Some(2)
            },
            value: Some(0)
        },

        Field {
            coordinates: Point {
                x: Some(1),
                y: Some(2)
            },
            value: Some(0)
        },
        Field {
            coordinates: Point {
                x: Some(2),
                y: Some(1)
            },
            value: Some(0)
        }];

        let fs_example: Vec<Field> = matrix_first_line.into_iter()
            .chain(matrix_second_line
                .into_iter())
            .chain(matrix_third_line
                .into_iter())
            .collect();


        let aps_example: Vec<u8> = vec![0, 2, 5, 7, 10, 14, 17, 19, 22, 24];

        return (matrix_example, fs_example, aps_example);
    }

    fn testing_data_no_road() -> (Vec<Vec<i8>>, Vec<Field>, Vec<u8>) {
        let matrix_example: Vec<Vec<i8>> = vec![
            vec![-1, 1], 
            vec![2, -1]
        ];

        let fs_example: Vec<Field> = vec![
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(1)
                },
                value: Some(1)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(0)
                },
                value: Some(2)
            },
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(0)
                },
                value: Some(-1)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(1)
                },
                value: Some(-1)
            },
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(0)
                },
                value: Some(-1)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(1)
                },
                value: Some(-1)
            },
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(1)
                },
                value: Some(1)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(0)
                },
                value: Some(2)
            },


        ];

        let aps_example: Vec<u8> = vec![0, 2, 4, 6, 8];

        return (matrix_example, fs_example, aps_example);
    }

    #[test]
    fn get_manhattan_distance_heuristic_test() {
        let start = Point { x: Some(0), y: Some(0) };
        let end = Point { x: Some(8), y: Some(8) };
        let data_sample = get_manhattan_distance_heuristic(start, end);
        assert_eq!(data_sample, 16);
    }

    #[test]
    fn fs_aps_from_matrix_test() {
        let sample_data = testing_data();
        assert_eq!(fs_aps_from_matrix(sample_data.0.clone()).unwrap(), (sample_data.1.clone(), sample_data.2.clone()));
    }

    #[test]
    fn get_element_childs_from_fs_aps_test() {
        let (_, fs, aps) = testing_data();
        let expected_result: Vec<Field> = vec![
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(0)
                },
                value: Some(0)
            },
            Field {
                coordinates: Point {
                    x: Some(1),
                    y: Some(1)
                },
                value: Some(2)
            }
        ];

        assert_eq!(get_element_childs_from_fs_aps(fs, aps, 1).unwrap(), expected_result);
    }

    #[test]
    fn get_field_by_index_test() {
        let (matrix, _, _) = testing_data();
        let index_field = Point {
            x: Some(1),
            y: Some(1)
        };
        let result_field = Field {
            coordinates: index_field.clone(),
            value: Some(2)
        };

        assert_eq!(get_field_by_index(matrix, index_field).unwrap(), result_field);
    }

    #[test]
    fn a_star_resolver_test() {
        let (_, fs, aps) = testing_data();

        let expected_output: Vec<Point> = vec![Point { x: Some(0), y: Some(1) }, Point { x: Some(1), y: Some(1) }];
        let start_end = (Field {
            coordinates: Point {
                x: Some(0),
                y: Some(1)
            },
            value: Some(1)
        }, Field {
            coordinates: Point {
                x: Some(1),
                y: Some(1)
            },
            value: Some(2)
        },);

        assert_eq!(a_star_resolver(fs, aps, 2, start_end).unwrap(), expected_output);
    }

    #[test]
    fn a_star_resolver_heavy_test() {
        let (_, fs, aps) = testing_data_heavy_matrix();

        let expected_output: Vec<Point> = vec![Point {
            x: Some(2),
            y: Some(0)
        },
        Point {
            x: Some(2),
            y: Some(1)
        },
        Point {
            x: Some(2),
            y: Some(2)
        },
        Point {
            x: Some(1),
            y: Some(2)
        },
        Point {
            x: Some(0),
            y: Some(2)
        },
        Point {
            x: Some(0),
            y: Some(1)
        },
        Point {
            x: Some(0),
            y: Some(0)
        }];

        let start_end = (
            Field {
                coordinates: Point {
                    x: Some(2),
                    y: Some(0)
                },
                value: Some(1)
            },
            Field {
                coordinates: Point{
                    x: Some(0),
                    y: Some(0)
                },
                value: Some(2)
            }
        );

        assert_eq!(a_star_resolver(fs, aps, 3, start_end).unwrap(), expected_output);
    }

    #[test]
    #[should_panic(expected = "It seem that it has no end to this level")]
    fn a_star_resolver_invalid_matrix() {
        let (_, fs, aps) = testing_data_no_road();
        let start_end = (
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(1)
                },
                value: Some(1)
            },
            Field {
                coordinates: Point{
                    x: Some(1),
                    y: Some(0)
                },
                value: Some(2)
            }
        );
        a_star_resolver(fs, aps, 2, start_end).unwrap();
    }

}