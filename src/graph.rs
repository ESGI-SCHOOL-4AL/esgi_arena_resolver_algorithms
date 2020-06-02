#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field {
    pub coordinates: Point,
    pub value: Option<i8>

}

impl Field {
    pub fn new() -> Self {
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
    pub fn new() -> Self {
        return Self {
            x: None,
            y: None
        };
    }

    pub fn get_index(&self, matrix_size: usize) -> Result<usize, &'static str> {
        if self.x.is_none() || self.y.is_none() || matrix_size == 0 {
            return Err("The x and y must be declare for get a index");
        }

        return Ok(self.x.unwrap() * matrix_size + self.y.unwrap());
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

pub fn get_start_to_end_points(matrix_bord: Vec<Vec<i8>>) -> Result<(Point, Point), &'static str> {
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

pub fn get_start_to_end_points_multi_roads(matrix_bord: Vec<Vec<i8>>) -> Result<(Point, Vec<Point>), &'static str> {
    let matrix_ligne_number = matrix_bord.len();
    let mut result_points: (Point, Vec<Point>) = (Point::new(), Vec::new());

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
                2 =>  result_points.1.push(Point { x: Some(i), y: Some(y) }),
                _ => continue
            }
        }
    }

    if result_points.0 == Point::new() || result_points.1.is_empty() {
        return Err("A start point and a end point are required");
    }

    return Ok(result_points);
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
/// use esgi_arena_resolver_algorithms::graph::Point;
/// use esgi_arena_resolver_algorithms::graph::Field;
/// use esgi_arena_resolver_algorithms::graph::fs_aps_from_matrix;
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
    let fs_end_index = aps.get(index + 1).unwrap().clone() as usize;

    println!("start: {}, end: {}", fs_start_index, fs_end_index);

    return Ok(fs.iter()
        .enumerate()
        .filter(|(index, _)| index >= &fs_start_index && index < &fs_end_index)
        .map(|(_, element)| element.clone())
        .collect());

}

pub fn remove_end_point_from_aps(fs: &mut Vec<Field>, end_point: Point) -> Vec<Field> {
    for i in 0..fs.len() {
        let mut current_field = fs.get_mut(i).unwrap();
        
        if current_field.coordinates == end_point {
            current_field.value = Some(0);
        }
    }

    return fs.to_vec();
}

#[cfg(test)]
mod test {
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
        fn remove_end_point_from_aps_test() {
            let mut fs_example: Vec<Field> = vec![
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
            let fs_expected: Vec<Field> = vec![
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
                    value: Some(0)
                },
                Field {
                    coordinates: Point {
                        x: Some(1),
                        y: Some(1)
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
            let end_point_to_purge = Point {
                x: Some(1),
                y: Some(1)
            };

            assert_eq!(remove_end_point_from_aps(&mut fs_example, end_point_to_purge), fs_expected);
        }
}