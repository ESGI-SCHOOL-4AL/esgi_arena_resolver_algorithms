pub fn chinese_rings_resolver(rings_number: usize) -> Vec<Vec<bool>> {
    let mut chinese_rings: Vec<bool> = Vec::with_capacity(rings_number);
    for _ in 0..rings_number + 1 {
        chinese_rings.push(false);
    }

    let mut mouvement_list: Vec<Vec<bool>> = vec![chinese_rings[1..].to_vec()];

    set_rings(&mut chinese_rings, rings_number, &mut mouvement_list);

    return mouvement_list;

}

fn give_ring(rings: &mut Vec<bool>, index: usize, mouvement_list: &mut Vec<Vec<bool>>) {
    rings[index] = true;
    mouvement_list.push(rings[1..].to_vec());
}

fn take_ring(rings: &mut Vec<bool>, index: usize, mouvement_list: &mut Vec<Vec<bool>>) {
    rings[index] = false;
    mouvement_list.push(rings[1..].to_vec());
}

fn set_rings(rings: &mut Vec<bool>, index: usize, mouvement_list: &mut Vec<Vec<bool>>) {
    if index == 0 {
        return;
    }

    if index == 1 {
        give_ring(rings, index, mouvement_list);
    
    }else {
        set_rings(rings, index - 1, mouvement_list);
        unset_rings(rings, index - 2, mouvement_list);
        give_ring(rings, index, mouvement_list);
        set_rings(rings, index - 2, mouvement_list);
    }


}

fn unset_rings(rings: &mut Vec<bool>, index: usize, mouvement_list: &mut Vec<Vec<bool>>) {
    if index == 0 {
        return;
    }

    if index == 1 {
        take_ring(rings, index, mouvement_list);
    
    }else {
        unset_rings(rings, index - 2, mouvement_list);
        take_ring(rings, index, mouvement_list);
        set_rings(rings, index - 2, mouvement_list);
        unset_rings(rings, index - 1, mouvement_list);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn give_ring_test() {
        let mut sample_data = vec![false, false, true, true];
        let expected_output = vec![false, true, true, true];
        give_ring(&mut sample_data, 1, &mut Vec::new());

        assert_eq!(sample_data, expected_output);
    }

    #[test]
    fn take_ring_test() {
        let mut sample_data = vec![false, false, true, true];
        let expected_output = vec![false, false, true, false];
        take_ring(&mut sample_data, 3, &mut Vec::new());

        assert_eq!(sample_data, expected_output);
    }

    #[test]
    fn set_rings_test() {
        let mut sample_data = vec![false, false, false, false, false];
        let expected_output = vec![
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
        let mut result: Vec<Vec<bool>> = Vec::new();
        set_rings(&mut sample_data, 4, &mut result);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn unset_rings_test() {
        let mut sample_data = vec![true, true, true, true, true];
        let expected_output = vec![
            vec![true, false, true, true],
            vec![false, false, true, true],
            vec![false, false, true, false],
            vec![true, false, true, false],
            vec![true, true, true, false],
            vec![false, true, true, false],
            vec![false, true, false, false],
            vec![true, true, false, false],
            vec![true, false, false, false],
            vec![false, false, false, false]
        ];
        let mut result: Vec<Vec<bool>> = Vec::new();
        unset_rings(&mut sample_data, 4, &mut result);

        assert_eq!(result, expected_output);
    }
    
}