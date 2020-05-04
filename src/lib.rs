pub mod algorithms {
    pub mod a_star {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Field {
            pub coordinates: Point,
            pub value: Option<i8>,
            pub move_cost: Option<u8>

        }

        impl Field {
            fn new() -> Self {
                return Self {
                    coordinates: Point::new(),
                    value: None,
                    move_cost: None
                };
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Point {
            pub x: Option<u8>,
            pub y: Option<u8>
        }

        impl Point {
            fn new() -> Self {
                return Self {
                    x: None,
                    y: None
                };
            }
        }

        fn bord_is_well_form(matrix_bord: &[Vec<i8>]) -> Option<&'static str> {
            let matrix_ligne_number = matrix_bord.len();

            if matrix_bord.is_empty() {
                return Some("The bord cannot be empty");
            }

            if matrix_ligne_number < 2 {
                return Some("The bord size cannot be shorter than 2 lignes");
            }

            if matrix_ligne_number > 20 {
                return Some("The bord size cannot be bigger than 20 lignes");
            }

            return None;
        }

        fn get_start_to_end_points(matrix_bord: Vec<Vec<i8>>) -> Result<(Point, Point), &'static str> {
            let matrix_ligne_number = matrix_bord.len();
            let mut result_points: (Point, Point) = (Point::new(), Point::new());

            let message_option = bord_is_well_form(matrix_bord.as_slice());

            if message_option.is_some() {
                return Err(message_option.unwrap());
            }

            for (i, matrix_line) in matrix_bord.iter().enumerate() {
                if matrix_line.len() != matrix_ligne_number {
                    return Err("The number of colunms should be equals to the number of lines");
                }

                for (y, point_value) in matrix_line.iter().enumerate() {
                    match point_value {
                        1 => if result_points.0 != Point::new() { 
                                return Err("Cannot have many start points"); 
                            } else {
                                result_points.0 = Point { x: Some(i as u8), y: Some(y as u8) };
                            },
                        2 => if result_points.1 != Point::new() { 
                                return Err("Cannot have many end points"); 
                            } else {
                                result_points.1 = Point { x: Some(i as u8), y: Some(y as u8) };
                            },
                        _ => continue
                    }
                }
            }

            if result_points.0 == Point::new() || result_points.1 == Point::new() {
                return Err("A start point and a end point are required");
            }

            return Ok(result_points);
        }

        /// Get heuristic value from start point to the target.
        /// 
        /// [For more explanations](https://xlinux.nist.gov/dads//HTML/manhattanDistance.html)
        /// 
        /// # Example
        /// 
        /// ```
        /// use esgi_arena_resolver_algorithms::algorithms::a_star::Point;
        /// use esgi_arena_resolver_algorithms::algorithms::a_star::get_manhattan_distance_heuristic;
        /// 
        /// let start = Point {
        ///     x: Some(0),
        ///     y: Some(0)
        /// };
        /// let end = Point {
        ///     x: Some(2),
        ///     y: Some(5)
        /// };
        /// 
        /// assert_eq!(get_manhattan_distance_heuristic(start, end), 7);
        /// ```
        pub fn get_manhattan_distance_heuristic(start_coordinates: Point, end_coordinates: Point) -> u8 {
            let x: i8 = start_coordinates.x.unwrap() as i8 - end_coordinates.x.unwrap() as i8;
            let y: i8 = start_coordinates.y.unwrap() as i8 - end_coordinates.y.unwrap() as i8;

            return (x.abs() + y.abs()) as u8;
        }

        /// Get a FS APS graph from bord matrix.
        /// 
        /// The FS vector contain all sons of each element.
        /// 
        /// The APS contain the range of FS index for each element.
        /// 
        /// # Example
        /// 
        /// ```
        /// use esgi_arena_resolver_algorithms::algorithms::a_star::Point;
        /// use esgi_arena_resolver_algorithms::algorithms::a_star::Field;
        /// use esgi_arena_resolver_algorithms::algorithms::a_star::fs_aps_from_matrix;
        ///
        /// let data_sample: Vec<Vec<i8>> = vec![
        ///     vec![0, 1], 
        ///     vec![0, 2]
        /// ];
        /// let expect_fs = vec![
        ///     Field {
        ///         coordinates: Point {
        ///             x: Some(0),
        ///             y: Some(1)
        ///         },
        ///         move_cost: None,
        ///         value: Some(1)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(1),
        ///             y: Some(0)
        ///         },
        ///         move_cost: None,
        ///         value: Some(0)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(0),
        ///             y: Some(0)
        ///         },
        ///         move_cost: None,
        ///         value: Some(0)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(1),
        ///             y: Some(1)
        ///         },
        ///         move_cost: None,
        ///         value: Some(2)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(1),
        ///             y: Some(1)
        ///         },
        ///         move_cost: None,
        ///         value: Some(2)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(0),
        ///             y: Some(0)
        ///         },
        ///         move_cost: None,
        ///         value: Some(0)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(1),
        ///             y: Some(0)
        ///         },
        ///         move_cost: None,
        ///         value: Some(0)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(0),
        ///             y: Some(1)
        ///         },
        ///         move_cost: None,
        ///         value: Some(1)
        ///      } 
        /// ];
        /// let expect_aps = vec![0, 2, 4, 6, 8];
        /// assert_eq!(fs_aps_from_matrix(data_sample).unwrap(), (expect_fs, expect_aps));
        /// ```
        pub fn fs_aps_from_matrix(matrix: Vec<Vec<i8>>) -> Result<(Vec<Field>, Vec<u8>), &'static str> {
            let message_option = bord_is_well_form(matrix.as_slice());

            if message_option.is_some() {
                return Err(message_option.unwrap());
            }

            let mut fs: Vec<Field> = Vec::new();
            let mut aps: Vec<u8> = vec![0];

            for line_index in 0..matrix.len() {
                for index in 0..matrix.len() {
                    let mut current_aps_index: u8 = 0;

                    if index as i8 - 1 >= 0 {
                        fs.push(Field {
                            coordinates: Point {
                                x: Some(line_index as u8),
                                y: Some(index as u8 - 1)
                            },
                            move_cost: None,
                            value: matrix.get(line_index).unwrap().get(index - 1).cloned()
                         });
                         current_aps_index += 1;
                    }

                    if index as i8 + 1 < matrix.len() as i8 {
                        fs.push(Field {
                            coordinates: Point {
                                x: Some(line_index as u8),
                                y: Some(index as u8 + 1)
                            },
                            move_cost: None,
                            value: matrix.get(line_index).unwrap().get(index + 1).cloned()
                         });
                         current_aps_index += 1;
                    }

                    if line_index as i8 - 1 >= 0 {
                        fs.push(Field {
                            coordinates: Point {
                                x: Some(line_index as u8 - 1),
                                y: Some(index as u8)
                            },
                            move_cost: None,
                            value: matrix.get(line_index - 1).unwrap().get(index).cloned()
                         });
                         current_aps_index += 1;
                    }

                    if line_index as i8 + 1 < matrix.len() as i8 {
                        fs.push(Field {
                            coordinates: Point {
                                x: Some(line_index as u8 + 1),
                                y: Some(index as u8)
                            },
                            move_cost: None,
                            value: matrix.get(line_index + 1).unwrap().get(index).cloned()
                         });
                         current_aps_index += 1;
                    }

                    aps.push(aps.last().unwrap() + current_aps_index);
                }
            }

            return Ok((fs, aps));
        }

        pub fn get_element_childs_from_fs_aps(fs: Vec<Field>, aps: Vec<u8>, index: usize) -> Result<Vec<Field>, &'static str> {
            if index >= aps.len() {
                return Err("The index cannot be bigger than the size of APS vector");
            }

            let fs_start_index = aps.get(index).unwrap().clone() as usize;
            let fs_end_index = (fs_start_index + 1) as usize;

            return Ok(fs.iter()
                .enumerate()
                .filter(|(index, _)| index >= &fs_start_index && index <= &fs_end_index)
                .map(|(_, element)| element.clone())
                .collect());

        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn bord_is_well_form_test() {
                let data_sample: Vec<Vec<i8>> = vec![
                    vec![0; 5],
                    vec![0, 1, 0, 0, 0], 
                    vec![0, 0, 2, 0, 0], 
                    vec![0; 5],
                    vec![0; 5]
                ];
                assert_eq!(bord_is_well_form(data_sample.as_slice()), None);
            }

            #[test]
            fn bord_is_well_form_empty() {
                let data_sample: Vec<Vec<i8>> = Vec::new();
                assert_eq!(bord_is_well_form(data_sample.as_slice()).unwrap(), "The bord cannot be empty");
            }

            #[test]
            fn bord_is_well_form_too_shorter() {
                let data_sample: Vec<Vec<i8>> = vec![
                    vec![0; 5],
                ];
                assert_eq!(bord_is_well_form(data_sample.as_slice()).unwrap(), "The bord size cannot be shorter than 2 lignes");
            }

            #[test]
            fn bord_is_well_form_too_bigger() {
                let data_sample: Vec<Vec<i8>> = vec![vec!(0; 21); 21];
                assert_eq!(bord_is_well_form(data_sample.as_slice()).unwrap(), "The bord size cannot be bigger than 20 lignes");
            }
            
            #[test]
            fn get_start_to_end_points_test() {
                let data_sample: Vec<Vec<i8>> = vec![
                    vec![0; 5],
                    vec![0, 1, 0, 0, 0], 
                    vec![0, 0, 2, 0, 0], 
                    vec![0; 5],
                    vec![0; 5]
                ];
                assert_eq!(get_start_to_end_points(data_sample).unwrap(), (Point { x: Some(1), y: Some(1) }, Point { x: Some(2), y: Some(2) }));
            }

            #[test]
            #[should_panic(expected = "The bord cannot be empty")]
            fn get_start_to_end_points_empty() {
                let data_sample: Vec<Vec<i8>> = Vec::new();
                get_start_to_end_points(data_sample).unwrap();
            }

            #[test]
            #[should_panic(expected = "The bord size cannot be bigger than 20 lignes")]
            fn get_start_to_end_points_too_bigger() {
                let data_sample: Vec<Vec<i8>> = vec![vec![0; 21]; 21];
                get_start_to_end_points(data_sample).unwrap();
            }

            #[test]
            #[should_panic(expected = "The bord size cannot be shorter than 2 lignes")]
            fn get_start_to_end_points_too_lower() {
                let data_sample: Vec<Vec<i8>> = vec![vec![0; 1]; 1];
                get_start_to_end_points(data_sample).unwrap();
            }

            #[test]
            #[should_panic(expected = "Cannot have many end points")]
            fn get_start_to_end_points_many_end() {
                let data_sample: Vec<Vec<i8>> = vec![
                    vec![0; 5],
                    vec![0, 1, 0, 0, 0], 
                    vec![0, 0, 2, 0, 0], 
                    vec![0, 0, 0, 0, 2],
                    vec![0; 5]
                ];
                get_start_to_end_points(data_sample).unwrap();
            }

            #[test]
            #[should_panic(expected = "Cannot have many start points")]
            fn get_start_to_end_points_many_start() {
                let data_sample: Vec<Vec<i8>> = vec![
                    vec![0; 5],
                    vec![0, 1, 0, 0, 0], 
                    vec![0, 0, 2, 0, 0], 
                    vec![1, 0, 0, 0, 0],
                    vec![0; 5]
                ];
                get_start_to_end_points(data_sample).unwrap();
            }
        }
    }

}
