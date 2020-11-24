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
    let mut array_length_and_repetions = increase_array_length_and_repetitions(0, 0);
    loop {
        if next_positions(
            array_length_and_repetions.0,
            array_length_and_repetions.1,
            0
        )
        .contains(&n)
        {
            return Name::Sheldon
        } else if next_positions(
            array_length_and_repetions.0,
            array_length_and_repetions.1,
            1
        )
        .contains(&n)
        {
            return Name::Leonard
        } else if next_positions(
            array_length_and_repetions.0,
            array_length_and_repetions.1,
            2
        )
        .contains(&n)
        {
            return Name::Penny
        } else if next_positions(
            array_length_and_repetions.0,
            array_length_and_repetions.1,
            3
        )
        .contains(&n)
        {
            return Name::Rajesh
        } else if next_positions(
            array_length_and_repetions.0,
            array_length_and_repetions.1,
            4
        )
        .contains(&n)
        {
            return Name::Howard
        } else {
            array_length_and_repetions = increase_array_length_and_repetitions(
                array_length_and_repetions.0,
                array_length_and_repetions.1,
            );
            if array_length_and_repetions.0 > n {
                break;
            }
        }
    }
    let mut cola_queue = Vec::new();
    for name in names {
        cola_queue.push(name);
    }
    if n == 1 {
        *cola_queue[n - 1]
    } else {
        *cola_queue[n - 1]
    }
}

fn next_positions(
    array_length: usize,
    nmb_of_repetitions: usize,
    initial_position: usize,
) -> Vec<usize> {
    let mut positions = Vec::new();
    let start = array_length + (nmb_of_repetitions * initial_position);
    let end = start + nmb_of_repetitions;
    for i in start..end {
        positions.push(i)
    }
    positions
}

fn increase_array_length_and_repetitions(
    array_length: usize,
    nmb_of_repetitions: usize,
) -> (usize, usize) {
    if array_length == 0 {
        (5, 2)
    } else {
        let length = (array_length * 2) + 5;
        let repetitions = nmb_of_repetitions * 2;
        (length, repetitions)
    }
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
    fn n_is_fifty_two() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];
        assert_eq!(who_is_next(names, 52), Name::Penny);
    }

    #[test]
    fn test_next_positions_first_sheldon() {
        assert_eq!(next_positions(5, 2, 0), [5, 6])
    }

    #[test]
    fn test_various_sheldon_positions() {
        assert_eq!(next_positions(15, 4, 0), [15, 16, 17, 18]);
        assert_eq!(next_positions(35, 8, 0), [35, 36, 37, 38, 39, 40, 41, 42])
    }

    #[test]
    fn test_various_penny_positions() {
        assert_eq!(next_positions(5, 2, 2), [9, 10]);
        assert_eq!(next_positions(15, 4, 2), [23, 24, 25, 26]);
        assert_eq!(next_positions(35, 8, 2), [51, 52, 53, 54, 55, 56, 57, 58])
    }

    #[test]
    fn test_increase_array_length_and_repetitions() {
        assert_eq!(increase_array_length_and_repetitions(0, 0), (5, 2));
    }

    #[test]
    fn test_various_increase_array_length_and_repetitions() {
        assert_eq!(increase_array_length_and_repetitions(5, 2), (15, 4));
        assert_eq!(increase_array_length_and_repetitions(15, 4), (35, 8));
    }
}
