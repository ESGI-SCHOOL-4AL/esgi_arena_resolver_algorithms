pub mod algorithms {
    pub mod a_star {
        #[derive(Debug, Clone, PartialEq)]
        pub struct AStarField {
            pub wrapped_field: Field,
            pub parent_field: Option<Box<Self>>,
            pub move_cost: Option<u8>
            
        }

        impl AStarField {
            fn new() -> Self {
                return Self {
                    wrapped_field: Field::new(),
                    parent_field: None,
                    move_cost: None
                };
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Field {
            pub coordinates: Point,
            pub value: Option<i8>

        }

        impl Field {
            fn new() -> Self {
                return Self {
                    coordinates: Point::new(),
                    value: None
                };
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Point {
            pub x: Option<usize>,
            pub y: Option<usize>
        }

        impl Point {
            fn new() -> Self {
                return Self {
                    x: None,
                    y: None
                };
            }

            fn get_index(&self) -> Result<usize, &'static str> {
                if self.x.is_none() || self.y.is_none() {
                    return Err("The x and y must be declare for get a index");
                }

                return Ok(self.x.unwrap() + self.y.unwrap());
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
                                result_points.0 = Point { x: Some(i), y: Some(y) };
                            },
                        2 => if result_points.1 != Point::new() { 
                                return Err("Cannot have many end points"); 
                            } else {
                                result_points.1 = Point { x: Some(i), y: Some(y) };
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
        /// let sample_data: Vec<Vec<i8>> = vec![
        ///     vec![0, 1], 
        ///     vec![0, 2]
        /// ];
        /// let expect_fs = vec![
        ///     Field {
        ///         coordinates: Point {
        ///             x: Some(0),
        ///             y: Some(1)
        ///         },
        ///         value: Some(1)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(1),
        ///             y: Some(0)
        ///         },
        ///         value: Some(0)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(0),
        ///             y: Some(0)
        ///         },
        ///         value: Some(0)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(1),
        ///             y: Some(1)
        ///         },
        ///         value: Some(2)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(1),
        ///             y: Some(1)
        ///         },
        ///         value: Some(2)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(0),
        ///             y: Some(0)
        ///         },
        ///         value: Some(0)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(1),
        ///             y: Some(0)
        ///         },
        ///         value: Some(0)
        ///      },
        ///      Field {
        ///         coordinates: Point {
        ///             x: Some(0),
        ///             y: Some(1)
        ///         },
        ///         value: Some(1)
        ///      } 
        /// ];
        /// let expect_aps = vec![0, 2, 4, 6, 8];
        /// assert_eq!(fs_aps_from_matrix(sample_data).unwrap(), (expect_fs, expect_aps));
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
                                x: Some(line_index),
                                y: Some(index - 1)
                            },
                            
                            value: matrix.get(line_index).unwrap().get(index - 1).cloned()
                         });
                         current_aps_index += 1;
                    }

                    if index as i8 + 1 < matrix.len() as i8 {
                        fs.push(Field {
                            coordinates: Point {
                                x: Some(line_index),
                                y: Some(index + 1)
                            },
                            
                            value: matrix.get(line_index).unwrap().get(index + 1).cloned()
                         });
                         current_aps_index += 1;
                    }

                    if line_index as i8 - 1 >= 0 {
                        fs.push(Field {
                            coordinates: Point {
                                x: Some(line_index - 1),
                                y: Some(index)
                            },
                            
                            value: matrix.get(line_index - 1).unwrap().get(index).cloned()
                         });
                         current_aps_index += 1;
                    }

                    if line_index as i8 + 1 < matrix.len() as i8 {
                        fs.push(Field {
                            coordinates: Point {
                                x: Some(line_index + 1),
                                y: Some(index)
                            },
                            
                            value: matrix.get(line_index + 1).unwrap().get(index).cloned()
                         });
                         current_aps_index += 1;
                    }

                    aps.push(aps.last().unwrap() + current_aps_index);
                }
            }

            return Ok((fs, aps));
        }

        pub fn get_field_by_index(matrix_bord: Vec<Vec<i8>>, index: Point) -> Result<Field, &'static str> {
            let matrix_ligne_number = matrix_bord.len();

            let message_option = bord_is_well_form(matrix_bord.as_slice());

            if message_option.is_some() {
                return Err(message_option.unwrap());
            }

            let mut result_field: Field = Field::new();

            for (i, matrix_line) in matrix_bord.iter().enumerate() {
                if matrix_line.len() != matrix_ligne_number {
                    return Err("The number of colunms should be equals to the number of lines");
                }

                if i != index.x.unwrap() {
                    continue;
                }

                result_field.coordinates.x = Some(i);

                for (y, value) in matrix_line.iter().enumerate() {
                    if y != index.y.unwrap() {
                        continue;
                    }

                    result_field.coordinates.y = Some(y);
                    result_field.value = Some(value.clone());
                } 
            }

            if index.x.is_none() || index.y.is_none() {
                return Err("The target point cannot be found inside the matrix");
            }

            return Ok(result_field);
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

        pub fn a_star_resolver(fs: Vec<Field>, aps: Vec<u8>, start_end_point: (Field, Field)) -> Result<Vec<Point>, &'static str> {
            if fs == Vec::new() || aps == Vec::new() || start_end_point == (Field::new(), Field::new()) {
                return Err("The parameters MUST be initializes");
            }

            let (start_point, end_point) = start_end_point;

            let start_field = AStarField {
                wrapped_field: start_point,
                parent_field: None,
                move_cost: Some(get_manhattan_distance_heuristic(start_point.coordinates, end_point.coordinates))
            };

            let mut open_list: Vec<AStarField> = vec![start_field];
            let mut close_list: Vec<AStarField> = Vec::new();
            let mut weight: u8 = 1;

            while ! open_list.is_empty() {
                quicksort(&mut open_list[..]);
                open_list.reverse();

                let current_a_star_field = open_list.pop();
                close_list.push(current_a_star_field.clone().unwrap());

                if current_a_star_field.clone().unwrap().wrapped_field.value.unwrap() == 2 {
                    return Ok(get_index_road_from_parents(current_a_star_field.unwrap()).unwrap());
                }

                let current_a_star_field_index = current_a_star_field.clone().unwrap().wrapped_field.coordinates.get_index();
                
                let current_a_star_field_childs = get_element_childs_from_fs_aps(fs.clone(), aps.clone(), current_a_star_field_index?);
                
                for child in current_a_star_field_childs? {
                    let a_star_child = AStarField {
                        wrapped_field: child,
                        move_cost: Some(weight + get_manhattan_distance_heuristic(current_a_star_field.clone().unwrap().wrapped_field.coordinates, end_point.coordinates)),
                        parent_field: Some(Box::new(current_a_star_field.clone().unwrap()))

                    };

                    open_list.push(a_star_child);
                }

                weight += 1;

            }
            

            return Err("Work in progress");
        }

        fn quicksort(to_sort: &mut [AStarField]) {
            if ! to_sort.is_empty() {
                let partition_index = quicksort_partition(to_sort);
                let to_sort_lenght = to_sort.len();
                
                quicksort(&mut to_sort[0..partition_index]);
                quicksort(&mut to_sort[partition_index + 1..to_sort_lenght]);
            }
        }

        fn quicksort_partition(to_partition: &mut [AStarField]) -> usize {
            let pivot = to_partition[to_partition.len() - 1].move_cost.unwrap();
            let mut x = 0;
    
            for i in 0..to_partition.len() - 1 {
                if to_partition[i].move_cost.unwrap() <= pivot {
                    to_partition.swap(x, i);
                    x += 1;
                }
            }
            
            to_partition.swap(x, to_partition.len() - 1);
            
            return x;
        }

        fn get_index_road_from_parents(mut final_a_star_field: AStarField) -> Option<Vec<Point>> {
            if final_a_star_field.parent_field.is_none() {
                return None;
            }

            let mut start_to_end_road: Vec<Point> = Vec::new();
            
            while final_a_star_field.parent_field.is_some() {
                let current_field = final_a_star_field.wrapped_field;
                start_to_end_road.push(current_field.coordinates);
                final_a_star_field = *final_a_star_field.parent_field.unwrap();
            }

            start_to_end_road.push(final_a_star_field.wrapped_field.coordinates);
            start_to_end_road.reverse();

            return Some(start_to_end_road);
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn bord_is_well_form_test() {
                let sample_data: Vec<Vec<i8>> = vec![
                    vec![0; 5],
                    vec![0, 1, 0, 0, 0], 
                    vec![0, 0, 2, 0, 0], 
                    vec![0; 5],
                    vec![0; 5]
                ];
                assert_eq!(bord_is_well_form(sample_data.as_slice()), None);
            }

            #[test]
            fn bord_is_well_form_empty() {
                let sample_data: Vec<Vec<i8>> = Vec::new();
                assert_eq!(bord_is_well_form(sample_data.as_slice()).unwrap(), "The bord cannot be empty");
            }

            #[test]
            fn bord_is_well_form_too_shorter() {
                let sample_data: Vec<Vec<i8>> = vec![
                    vec![0; 5],
                ];
                assert_eq!(bord_is_well_form(sample_data.as_slice()).unwrap(), "The bord size cannot be shorter than 2 lignes");
            }

            #[test]
            fn bord_is_well_form_too_bigger() {
                let sample_data: Vec<Vec<i8>> = vec![vec!(0; 21); 21];
                assert_eq!(bord_is_well_form(sample_data.as_slice()).unwrap(), "The bord size cannot be bigger than 20 lignes");
            }
            
            #[test]
            fn get_start_to_end_points_test() {
                let sample_data: Vec<Vec<i8>> = vec![
                    vec![0; 5],
                    vec![0, 1, 0, 0, 0], 
                    vec![0, 0, 2, 0, 0], 
                    vec![0; 5],
                    vec![0; 5]
                ];
                assert_eq!(get_start_to_end_points(sample_data).unwrap(), (Point { x: Some(1), y: Some(1) }, Point { x: Some(2), y: Some(2) }));
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
            fn quicksort_test() {
                let mut sample_data: Vec<AStarField> = vec![
                    AStarField { wrapped_field: Field::new(), parent_field: None, move_cost: Some(10) },
                    AStarField { wrapped_field: Field::new(), parent_field: None, move_cost: Some(11) },
                    AStarField { wrapped_field: Field::new(), parent_field: None, move_cost: Some(9) },
                    AStarField { wrapped_field: Field::new(), parent_field: None, move_cost: Some(15) }
                ];
                let expected_output = vec![
                    AStarField { wrapped_field: Field::new(), parent_field: None, move_cost: Some(9) },
                    AStarField { wrapped_field: Field::new(), parent_field: None, move_cost: Some(10) },
                    AStarField { wrapped_field: Field::new(), parent_field: None, move_cost: Some(11) },
                    AStarField { wrapped_field: Field::new(), parent_field: None, move_cost: Some(15) }
                ];
                quicksort(&mut sample_data[..]);

                assert_eq!(sample_data, expected_output);
            }

            #[test]
            fn get_parents_list_test() {
                let start_element = Box::new(AStarField { wrapped_field: Field { 
                        coordinates: Point { x: Some(0), y: Some(1) }, 
                        value: Some(1) 
                    }, 
                    parent_field: None, 
                    move_cost: None 
                });
                let road_element = Box::new(AStarField { wrapped_field: Field { 
                        coordinates: Point { x: Some(0), y: Some(0) }, 
                        value: Some(1) 
                    }, 
                    parent_field: Some(start_element), 
                    move_cost: None 
                });
                let road_end = Box::new(AStarField { wrapped_field: Field { 
                        coordinates: Point { x: Some(1), y: Some(1) }, 
                        value: Some(1) 
                    }, 
                    parent_field: Some(road_element), 
                    move_cost: None 
                });

                let expected_output: Vec<Point> = vec![Point { x: Some(0), y: Some(1) }, Point { x: Some(0), y: Some(0) }, Point { x: Some(1), y: Some(1) }];

                assert_eq!(get_index_road_from_parents(*road_end).unwrap(), expected_output);
            }
        }
    }

}
