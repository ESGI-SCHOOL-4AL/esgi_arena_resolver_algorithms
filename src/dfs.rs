pub use crate::graph::{Field, Point, get_element_childs_from_fs_aps};

fn dfs_fs_aps(fs: Vec<Field>, aps: Vec<u8>, start: Field, matrix_size: usize) -> Vec<Vec<Field>> {
    let mut to_show: Vec<Vec<Field>> = Vec::new();
    let mut discover: Vec<Field> = Vec::new();
    let mut undiscover: Vec<Field> = vec![start];

    while ! undiscover.is_empty() {
        let current_field = undiscover.pop().unwrap();
        let mut is_discover = false;
        let mut current_path: Vec<Field> = Vec::new();

        for discover_field in &discover {
            if current_field == *discover_field {
                is_discover = true;
            }
        }

        if ! is_discover {
            discover.push(current_field);
            current_path.push(current_field);

            let current_field_childs = get_element_childs_from_fs_aps(fs.clone(), aps.clone(), current_field.coordinates.get_index(matrix_size).unwrap());
            for child in current_field_childs.unwrap() {
                undiscover.push(child);
            }
        }
    }

    return to_show;

}

fn dfs_fs_aps_recursive(fs: Vec<Field>, aps: Vec<u8>, start: Field, matrix_size: usize, discovered: &mut Vec<Field>) {
    discovered.push(start);
    println!("{:?}", start);

    for child in get_element_childs_from_fs_aps(fs.clone(), aps.clone(), start.coordinates.get_index(matrix_size).unwrap()).unwrap() {
        let mut is_discover = false;
        for discovered_field in discovered.to_owned() {
            if discovered_field == child {
                is_discover = true;
            }
        }
        if is_discover {
            continue;
        }

        dfs_fs_aps_recursive(fs.clone(), aps.clone(), child, matrix_size, discovered);
    }

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dfs_fs_aps_test() {
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
        let start_field_example = Field {
            coordinates: Point {
                x: Some(0),
                y: Some(1)
            },
            value: Some(1)
        };

        for element in dfs_fs_aps(fs_example, aps_example, start_field_example, 2) {
            println!("{:?}", element);
        }

        assert_eq!(2, 4);
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
        let aps_example: Vec<u8> = vec![0, 2, 4, 6, 8];
        let start_field_example = Field {
            coordinates: Point {
                x: Some(0),
                y: Some(1)
            },
            value: Some(1)
        };

        dfs_fs_aps_recursive(fs_example, aps_example, start_field_example, 2, &mut Vec::new());

        assert_eq!(2, 4);
        
    }
}