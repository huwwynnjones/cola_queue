/// _BigBang_ gang
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Name {
    Sheldon,
    Leonard,
    Penny,
    Rajesh,
    Howard,
}

/// Just to make code look a bit nicer
type Names = Vec<Name>;

/// Will return the `Name` of the person who will drink the `n`-th cola.
fn who_is_next(names: &Names, n: usize) -> Name {
    let mut cola_queue = Vec::new();
    for name in names {
        cola_queue.push(name);
    }
    for name in names {
        cola_queue.push(name);
        cola_queue.push(name);
    }
    if n == 1 {
        *cola_queue[n-1]
    } else {
        *cola_queue[n-1]
    }
}

fn next_positions(array_length: usize, nmb_of_repetitions: usize, initial_idx: usize) -> Vec<usize> {
    let mut positions = Vec::new();
    let start = array_length + (nmb_of_repetitions * initial_idx);
    let end = start + nmb_of_repetitions;
    for i in start..end {
        positions.push(i)
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_is_one() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];
        assert_eq!(who_is_next(names, 1), Name::Sheldon);
    }

    #[test]
    fn n_is_two() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];
        assert_eq!(who_is_next(names, 2), Name::Leonard);
    }

    #[test]
    fn n_is_eight() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];
        assert_eq!(who_is_next(names, 8), Name::Leonard);
    }

    #[test]
    fn test_next_positions_first_sheldon() {
        assert_eq!(next_positions(5, 2, 0), [5,6])
    }
}
