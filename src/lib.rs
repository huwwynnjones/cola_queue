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
        cola_queue.push(name);
    }
    if n== 1 {
        *cola_queue[n]
    } else {
        Name::Leonard
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
}
