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
    let mut array_length = names.len();
    let mut nmb_name_repetitions = 2;
    while array_length <= n {
        for (i, name) in names.iter().enumerate() {
            if within_position_range(array_length, nmb_name_repetitions, i, n) {
                return *name;
            }
        }
        array_length = (array_length * 2) + names.len();
        nmb_name_repetitions *= 2;
    }
    names[n - 1]
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
    fn n_is_eight_small_list() {
        let names = &vec![Name::Sheldon, Name::Leonard];
        assert_eq!(who_is_next(names, 8), Name::Sheldon);
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
}
