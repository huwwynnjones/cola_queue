# Double Cola

This is a kata on code wars. This was an attempt to follow a TDD approach with micro-commits.

Bit by bit the solution was reached and the commit history reflects how the tests and ugly initial code slowly evolve
into a working solution. The end result is not as simple and clever as the ideal solution. The best solution reflects a
better understanding of maths, and I don't think that I could have reached it through TDD alone. I would have had to go through
the maths on paper to reach it.

Here is the best solution. It relies on recognising that n can be worked backwards, reducing the array-length until it is back
until the stating size to find the correct index number.

```
fn who_is_next(names: &Names, n: usize) -> Name {
    let len = names.len();
    let mut r = n - 1;
    
    while r >= len {
        r = (r - len) / 2;
    }
    
    names[r]
}
```