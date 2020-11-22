/// _BigBang_ gang
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Name { Sheldon, Leonard, Penny, Rajesh, Howard }

/// Just to make code look a bit nicer
type Names = Vec<Name>;

/// Will return the `Name` of the person who will drink the `n`-th cola.
fn who_is_next(names: &Names, n: usize) -> Name {
    todo!("Your code goes here!")
}

#[cfg(test)]
mod tests {
    
}