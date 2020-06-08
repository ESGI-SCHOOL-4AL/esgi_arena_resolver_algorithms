//! # Description
//! This the A* algorithme module.
//! [For more explainations](https://xlinux.nist.gov/dads//HTML/manhattanDistance.html)

pub use crate::graph::{Point, Field, get_start_to_end_points, get_start_to_end_points_multi_roads, get_element_childs_from_fs_aps, remove_end_point_from_aps};

/// Struct for handle A* algotithm interaction.
/// It's compose of the target field, the parent field and the sum between the heuristic 
/// and the cost to move on the target field.
/// 
/// # Example
/// 
/// ```
/// use esgi_arena_resolver_algorithms::a_star::AStarField;
/// use esgi_arena_resolver_algorithms::graph::{ Point, Field };
/// 
/// let mut a_star_example = AStarField::new();
/// 
/// a_star_example.wrapped_field = Field {
///     coordinates: Point {
///         x: Some(1),
///         y: Some(0)
///     },
///     value: Some(0)
/// };
/// 
/// a_star_example.parent_field = Some(Box::new(AStarField { 
///     wrapped_field: Field {
///             coordinates: Point {
///                 x: Some(0),
///                 y: Some(0)
///             },
///             value: Some(1)
///     },
///     parent_field: None,
///     move_cost: Some(9)
/// 
/// }));
/// 
/// a_star_example.move_cost = Some(10);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AStarField {
    pub wrapped_field: Field,
    pub parent_field: Option<Box<Self>>,
    pub move_cost: Option<u8>
    
}

impl AStarField {
    pub fn new() -> Self {
        return Self {
            wrapped_field: Field::new(),
            parent_field: None,
            move_cost: None
        };
    }
}

/// Get heuristic value from start point to the target.
/// 
/// [For more explanations](https://xlinux.nist.gov/dads//HTML/manhattanDistance.html)
/// 
/// # Example
/// 
/// ```
/// use esgi_arena_resolver_algorithms::graph::Point;
/// use esgi_arena_resolver_algorithms::a_star::get_manhattan_distance_heuristic;
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

pub fn a_star_multi_roads_resolver(fs: &mut Vec<Field>, aps: Vec<u8>, matrix_size: usize, start_end_point: (Field, Vec<Field>)) -> Result<Vec<Vec<Point>>, &'static str> {
    if fs.is_empty() || aps.is_empty() || matrix_size == 0 || start_end_point == (Field::new(), Vec::new()) {
        return Err("The parameters MUST be initializes");
    }

    let (start_point, end_points) = start_end_point;
    let mut list_of_roads: Vec<Vec<Point>> = Vec::new();
    
    for end_point in end_points {
        list_of_roads.push(a_star_resolver(fs.clone(), aps.clone(), matrix_size, (start_point, end_point))?); 
        *fs = remove_end_point_from_aps(fs, end_point.coordinates);
    }

    return Ok(list_of_roads);
}

/// A* resolver function.
/// It's find the shorter path between two points of a graph.
/// 
/// [For more explainations](https://en.wikipedia.org/wiki/A*_search_algorithm)
/// 
/// # Example
/// 
/// ```
/// use esgi_arena_resolver_algorithms::graph::{ Point, Field };
/// use esgi_arena_resolver_algorithms::a_star::a_star_resolver;
/// 
/// let matrix_example: Vec<Vec<i8>> = vec![
/// vec![0, 1], 
/// vec![0, 2]
/// ];

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

/// let aps_example: Vec<u8> = vec![0, 2, 4, 6, 8];
/// let start_end_fields = (Field {
///     coordinates: Point {
///         x: Some(0),
///         y: Some(1)
///     },
///     value: Some(1)
///     },
///     Field {
///         coordinates: Point {
///             x: Some(1),
///             y: Some(1)
///         },
///         value: Some(2)
///     }
/// );
/// 
/// assert_eq!(a_star_resolver(fs_example, aps_example, matrix_example.len(), start_end_fields).unwrap(), vec![
///     Point {
///         x: Some(0),
///         y: Some(1)
///     },
///     Point {
///         x: Some(1),
///         y: Some(1)
///     }
/// ]);
/// ```
pub fn a_star_resolver(fs: Vec<Field>, aps: Vec<u8>, matrix_size: usize, start_end_point: (Field, Field)) -> Result<Vec<Point>, &'static str> {
    if fs.is_empty() || aps.is_empty() || start_end_point == (Field::new(), Field::new()) {
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

        let current_a_star_field_index = current_a_star_field.clone().unwrap().wrapped_field.coordinates.get_index(matrix_size);
        let current_a_star_field_childs: Vec<Field> = get_element_childs_from_fs_aps(fs.clone(), aps.clone(), current_a_star_field_index?)?;

        for child in current_a_star_field_childs {
            let mut is_invalid_son = false;

            if child.value.unwrap() == -1 {
                is_invalid_son = true;
            }

            for closed_field in close_list.clone() {
                if closed_field.wrapped_field == child {
                    is_invalid_son = true;
                }
            }

            let a_star_child = AStarField {
                wrapped_field: child.clone(),
                move_cost: Some(weight + get_manhattan_distance_heuristic(child.clone().coordinates, end_point.coordinates)),
                parent_field: Some(Box::new(current_a_star_field.clone().unwrap()))

            };

            for opened_field in open_list.clone() {
                if opened_field.wrapped_field == child 
                    && a_star_child.move_cost.unwrap() < opened_field.move_cost.unwrap(){
                    is_invalid_son = true;
                }
            }

            if is_invalid_son {
                continue;
            }

            open_list.push(a_star_child);
        }

        weight += 1;

    }
    
    return Err("It seem that it has no end to this level");
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
