//! # Description
//! This a DFS handling module.
//! It's a implementarion DFS algorithm for find all paths between two points of a graph.
//! [For more explainations](https://en.wikipedia.org/wiki/Depth-first_search) 

pub use crate::graph::{Field, Point, get_element_childs_from_fs_aps};

/// Get all paths between two Point of a graph.
///
/// # Example
/// 
/// ```
/// use esgi_arena_resolver_algorithms::graph::{Field, Point, get_element_childs_from_fs_aps};
/// use esgi_arena_resolver_algorithms::dfs::dfs_fs_aps_recursive;
/// 
/// let fs_example: Vec<Field> = vec![
/// Field {
///     coordinates: Point {
///         x: Some(0),
///         y: Some(1)
///     },
///     value: Some(1)
/// },
/// Field {
///     coordinates: Point {
///         x: Some(1),
///         y: Some(0)
///     },
///     value: Some(0)
/// },
/// Field {
///     coordinates: Point {
///         x: Some(0),
///         y: Some(0)
///     },
///     value: Some(0)
/// },
/// Field {
///     coordinates: Point {
///         x: Some(1),
///         y: Some(1)
///     },
///     value: Some(2)
/// },
/// Field {
///     coordinates: Point {
///         x: Some(1),
///         y: Some(1)
///     },
///     value: Some(2)
/// },
/// Field {
///     coordinates: Point {
///         x: Some(0),
///         y: Some(0)
///     },
///     value: Some(0)
/// },
/// Field {
///     coordinates: Point {
///         x: Some(1),
///         y: Some(0)
///     },
///     value: Some(0)
/// },
/// Field {
///     coordinates: Point {
///         x: Some(0),
///         y: Some(1)
///     },
///     value: Some(1)
/// }
/// ];
/// let aps_example: Vec<u32> = vec![0, 2, 4, 6, 8];
/// 
/// let start_field_example = Field {
/// coordinates: Point {
///     x: Some(0),
///     y: Some(1)
/// },
/// value: Some(1)
/// };
/// let end_field_example = Field {
/// coordinates: Point {
///     x: Some(1),
///     y: Some(1)
/// },
/// value: Some(2)
/// };
/// let expected_output = vec![vec![Field { coordinates: Point { x: Some(0), y: Some(1) }, value: Some(1) },
/// Field { coordinates: Point { x: Some(0), y: Some(0) }, value: Some(0) },
/// Field { coordinates: Point { x: Some(1), y: Some(0) }, value: Some(0) },
/// Field { coordinates: Point { x: Some(1), y: Some(1) }, value: Some(2) }
/// ],
/// vec![Field { coordinates: Point { x: Some(0), y: Some(1) }, value: Some(1) },
/// Field { coordinates: Point { x: Some(1), y: Some(1) }, value: Some(2) }
/// ]];
/// 
/// let mut all_path: Vec<Vec<Field>> = Vec::new();
/// 
/// dfs_fs_aps_recursive(fs_example, aps_example, (start_field_example, end_field_example), 2, &mut Vec::new(), &mut vec![start_field_example], &mut all_path);
/// 
/// assert_eq!(all_path, expected_output);
/// ```
pub fn dfs_fs_aps_recursive(fs: Vec<Field>, aps: Vec<u32>, start_end: (Field, Field), matrix_size: usize, discovered: &mut Vec<Field>, current_path: &mut Vec<Field>, all_path: &mut Vec<Vec<Field>>) {
    let (start, end) = start_end;
    discovered.push(start);

    let current_field_discover_index = discovered.len() - 1;

    if start == end {
        all_path.push(current_path.to_owned());
        return;
    }

    for child in get_element_childs_from_fs_aps(fs.clone(), aps.clone(), start.coordinates.get_index(matrix_size).unwrap()).unwrap() {
        let mut is_discover = false;
        
        if child.value.unwrap() == -1 {
            continue;
        }

        for discovered_field in discovered.to_owned() {
            if discovered_field == child {
                is_discover = true;
            }
        }

        if is_discover {
            continue;
        }

        current_path.push(child);
        let current_field_index_path = current_path.len() - 1;
        dfs_fs_aps_recursive(fs.clone(), aps.clone(), (child, end), matrix_size, &mut discovered.clone(), &mut current_path.clone(), all_path);
        current_path.remove(current_field_index_path);
    }

    discovered.remove(current_field_discover_index);

}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn dfs_fs_aps_recursive_test() {
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

        let start_field_example = Field {
            coordinates: Point {
                x: Some(0),
                y: Some(1)
            },
            value: Some(1)
        };
        let end_field_example = Field {
            coordinates: Point {
                x: Some(1),
                y: Some(1)
            },
            value: Some(2)
        };
        let expected_output = vec![vec![Field { coordinates: Point { x: Some(0), y: Some(1) }, value: Some(1) },
        Field { coordinates: Point { x: Some(0), y: Some(0) }, value: Some(0) },
        Field { coordinates: Point { x: Some(1), y: Some(0) }, value: Some(0) },
        Field { coordinates: Point { x: Some(1), y: Some(1) }, value: Some(2) }
        ],
        vec![Field { coordinates: Point { x: Some(0), y: Some(1) }, value: Some(1) },
        Field { coordinates: Point { x: Some(1), y: Some(1) }, value: Some(2) }
        ]];
        

        let mut all_path: Vec<Vec<Field>> = Vec::new();

        dfs_fs_aps_recursive(fs_example, aps_example, (start_field_example, end_field_example), 2, &mut Vec::new(), &mut vec![start_field_example], &mut all_path);

        assert_eq!(all_path, expected_output);
        
    }

    #[test]
    fn dfs_fs_aps_recursive_heavy_test() {
        let (_, fs_example, aps_example) = testing_data_heavy_matrix();

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

        let expected_output = vec![vec![Field { coordinates: Point { x: Some(2), y: Some(0) }, value: Some(1) },
        Field { coordinates: Point { x: Some(2), y: Some(1) }, value: Some(0) },
        Field { coordinates: Point { x: Some(2), y: Some(2) }, value: Some(0) },
        Field { coordinates: Point { x: Some(1), y: Some(2) }, value: Some(0) },
        Field { coordinates: Point { x: Some(0), y: Some(2) }, value: Some(0) },
        Field { coordinates: Point { x: Some(0), y: Some(1) }, value: Some(0) },
        Field { coordinates: Point { x: Some(0), y: Some(0) }, value: Some(2) }]];
        
        let mut all_path: Vec<Vec<Field>> = Vec::new();

        dfs_fs_aps_recursive(fs_example, aps_example, start_end, 3, &mut Vec::new(), &mut vec![start_end.0], &mut all_path);

        assert_eq!(all_path, expected_output);
        
    }


}