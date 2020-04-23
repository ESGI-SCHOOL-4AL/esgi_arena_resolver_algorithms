
pub mod algorithms {
    pub mod a_star {

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

        fn get_start_to_end_points(matrix_bord: Vec<Vec<i8>>) -> Result<[[u8; 2]; 2], &'static str> {
            let matrix_ligne_number = matrix_bord.len();
            let mut result_points: [[u8; 2]; 2] = [[0; 2], [0; 2]];

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
                        1 => if result_points[0] != [0; 2] { 
                                return Err("Cannot have many start points"); 
                            } else {
                                result_points[0] = [i as u8, y as u8];
                            },
                        2 => if result_points[1] != [0; 2] { 
                                return Err("Cannot have many end points"); 
                            } else {
                                result_points[1] = [i as u8, y as u8];
                            },
                        _ => continue
                    }
                }
            }

            if matrix_bord[0] == [0; 2] || matrix_bord[1] == [0; 2] {
                return Err("A start point and a end point are required");
            }

            return Ok(result_points);
        }

        /// Get heuristic value from start point too the target
        /// [For more explanations](https://xlinux.nist.gov/dads//HTML/manhattanDistance.html)
        /// 
        /// # Exemple
        /// 
        /// ```
        /// use esgi_arena_resolver_algorithms::algorithms::a_star::get_manhattan_distance_heuristic;
        /// 
        /// let mut data_to_send: [[u8; 2]; 2] = [[0; 2]; 2]; 
        /// let current_point: [u8; 2] = [0, 0];
        /// let target_point: [u8; 2] = [2, 5];
        /// 
        /// data_to_send[0] = current_point;
        /// data_to_send[1] = target_point;
        /// 
        /// assert_eq!(get_manhattan_distance_heuristic(data_to_send), 7);
        /// ```
        pub fn get_manhattan_distance_heuristic(start_to_end_index: [[u8; 2]; 2]) -> u8 {
            let x: i8 = start_to_end_index[0][0] as i8 - start_to_end_index[1][0] as i8;
            let y: i8 = start_to_end_index[0][1] as i8 - start_to_end_index[1][1] as i8;

            return (x.abs() + y.abs()) as u8;
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
                assert_eq!(get_start_to_end_points(data_sample).unwrap(), [[1, 1], [2, 2]]);
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
