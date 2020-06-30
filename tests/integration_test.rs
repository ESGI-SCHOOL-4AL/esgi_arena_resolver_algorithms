#[cfg(test)]
mod tests {
    use esgi_arena_resolver_algorithms::a_star::*;
    use esgi_arena_resolver_algorithms::graph::*;
    use esgi_arena_resolver_algorithms::chinese_rings::*;
    
    fn testing_data() -> (Vec<Vec<i8>>, Vec<Field>, Vec<u32>) {
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

        let aps_example: Vec<u32> = vec![0, 2, 4, 6, 8];

        return (matrix_example, fs_example, aps_example);
    }

    fn testing_data_heavy_matrix() -> (Vec<Vec<i8>>, Vec<Field>, Vec<u32>) {
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


        let aps_example: Vec<u32> = vec![0, 2, 5, 7, 10, 14, 17, 19, 22, 24];

        return (matrix_example, fs_example, aps_example);
    }

    fn testing_data_heavy_matrix_multi_end() -> (Vec<Vec<i8>>, Vec<Field>, Vec<u32>) {
        let matrix_example: Vec<Vec<i8>> = vec![
            vec![2, 0, 0], 
            vec![-1, -1, 0],
            vec![1, 0, 2]
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
            value: Some(2)
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
            value: Some(2)
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


        let aps_example: Vec<u32> = vec![0, 2, 5, 7, 10, 14, 17, 19, 22, 24];

        return (matrix_example, fs_example, aps_example);
    }

    

    fn testing_data_no_road() -> (Vec<Vec<i8>>, Vec<Field>, Vec<u32>) {
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

        let aps_example: Vec<u32> = vec![0, 2, 4, 6, 8];

        return (matrix_example, fs_example, aps_example);
    }

    #[test]
    #[should_panic(expected = "The bord cannot be empty")]
    fn get_start_to_end_points_empty() {
        let sample_data: Vec<Vec<i8>> = Vec::new();
        get_start_to_end_points(sample_data).unwrap();
    }

    #[test]
    #[should_panic(expected = "The bord size cannot be bigger than 20 lignes")]
    fn get_start_to_end_points_too_bigger() {
        let sample_data: Vec<Vec<i8>> = vec![vec![0; 21]; 21];
        get_start_to_end_points(sample_data).unwrap();
    }

    #[test]
    #[should_panic(expected = "The bord size cannot be shorter than 2 lignes")]
    fn get_start_to_end_points_too_lower() {
        let sample_data: Vec<Vec<i8>> = vec![vec![0; 1]; 1];
        get_start_to_end_points(sample_data).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot have many end points")]
    fn get_start_to_end_points_many_end() {
        let sample_data: Vec<Vec<i8>> = vec![
            vec![0; 5],
            vec![0, 1, 0, 0, 0], 
            vec![0, 0, 2, 0, 0], 
            vec![0, 0, 0, 0, 2],
            vec![0; 5]
        ];
        get_start_to_end_points(sample_data).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot have many start points")]
    fn get_start_to_end_points_many_start() {
        let sample_data: Vec<Vec<i8>> = vec![
            vec![0; 5],
            vec![0, 1, 0, 0, 0], 
            vec![0, 0, 2, 0, 0], 
            vec![1, 0, 0, 0, 0],
            vec![0; 5]
        ];
        get_start_to_end_points(sample_data).unwrap();
    }

    #[test]
    fn get_start_to_end_points_multi_roads_test() {
        let sample_data: Vec<Vec<i8>> = vec![
            vec![0; 5],
            vec![0, -1, -1, -1, 2], 
            vec![0, 0, 2, 0, 0], 
            vec![1, -1, 0, 0, 2],
            vec![0; 5]
        ];
        let start_point = Point { x: Some(3), y: Some(0) };
        let end_points = vec![Point {
            x: Some(1),
            y: Some(4)
        },
        Point {
            x: Some(2),
            y: Some(2)
        },
        Point {
            x: Some(3),
            y: Some(4)
        }
        ];
        let expected_output: (Point, Vec<Point>) = (start_point, end_points);

        assert_eq!(get_start_to_end_points_multi_roads(sample_data).unwrap(), expected_output);
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
        });

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

    #[test]
    fn a_star_multi_roads_resolver_test() {
        let (_, mut fs, aps) = testing_data_heavy_matrix_multi_end();
        let start_end = (Field {
            coordinates: Point {
                x: Some(2),
                y: Some(0)
            },
            value: Some(1)
        }, vec![Field {
                coordinates: Point {
                    x: Some(2),
                    y: Some(2)
                },
                value: Some(2)
            },
            Field {
                coordinates: Point {
                    x: Some(0),
                    y: Some(0)
                },
                value: Some(2)
            }
        ]);

        let expected_output = vec![vec![
            Point {
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
            }
        ], 
        vec![
            Point {
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
            }]
        ];

        assert_eq!(a_star_multi_roads_resolver(&mut fs, aps, 3, start_end).unwrap(), expected_output);
    }

    #[test]
    fn chinese_rings_resolver_test() {
        let size: usize = 4;
        let expected_output = vec![
            vec![false, false, false, false],
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, true, false, false],
            vec![false, true, true, false],
            vec![true, true, true, false],
            vec![true, false, true, false],
            vec![false, false, true, false],
            vec![false, false, true, true],
            vec![true, false, true, true],
            vec![true, true, true, true]
        ];

        assert_eq!(chinese_rings_resolver(size), expected_output);
    }

}