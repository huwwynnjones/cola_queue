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
    while array_length_and_repetions.0 <= n {
        for (i, name) in names.iter().enumerate() {
            if within_position_range(
                array_length_and_repetions.0,
                array_length_and_repetions.1,
                i,
                n,
            ) {
                return *name;
            }
        }
        array_length_and_repetions = increase_array_length_and_repetitions(
            array_length_and_repetions.0,
            array_length_and_repetions.1,
        );
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

fn within_position_range(
    array_length: usize,
    nmb_of_repetitions: usize,
    initial_position: usize,
    n: usize,
) -> bool {
    let start = array_length + (nmb_of_repetitions * initial_position);
    let end = start + nmb_of_repetitions - 1;
    n >= start && n <= end
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
    fn n_is_very_large() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];
        assert_eq!(who_is_next(names, 7_230_702_951), Name::Leonard);
    }

    #[test]
    fn n_is_three_hundred_and_seventeen() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];
        assert_eq!(who_is_next(names, 317), Name::Sheldon);
    }

    #[test]
    fn test_within_position_range() {
        assert!(within_position_range(5, 2, 0, 5));
        assert!(within_position_range(15, 4, 2, 25));
        assert!(within_position_range(35, 8, 2, 56));
        assert!(within_position_range(35, 8, 2, 56));
        assert!(!within_position_range(35, 8, 2, 89));
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
