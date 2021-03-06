// Important to understand the range of the for loops:
// In rust, the range n..m means every value from n to m-1.
// So if you write for i in 0..100, i will start at zero and go to 99.
// Therefore you will see a lot of (n..m+1)'s in this code,
// which means from n to m, inclusive.

fn main() {
    let s1 = 5;
    let s2 = 50;
    println!("{} spaces: {}", s1, possible_choices(s1));
    println!("{} spaces: {}", s2, possible_choices(s2));
}

// This code produces the right answers for s=1, s=2, ..., s=6,
// but for some reason it produces the wrong value for s=50.
// The value I end up with is almost 10 times too big.

fn possible_choices(spaces: u64) -> u64
{
    let mut ans = 0;

    for block_size in 2..4+1 {
        // We iterate from 1 to the maximum number of blocks we can fit
        // in the given number of spaces
        for r in 1..(spaces/block_size)+1 {
            // We're going to compute n C r (n choose r).
            // The r values count up from 1.
            // The n values need to count down from spaces - 1,
            // in increments of block_size-1.
            let n = spaces - r * (block_size - 1);
            // We successively add our n C r values to find the total number
            // of possible block combinations.
            ans += choose(n, r);
        }
    }
    ans
}

fn choose(n: u64, r: u64) -> u64
{
    if r > n {
        return 0;
    }
    // if r > n/2, choose(n, r) == choose(n, n - r),
    // but choose(n, r) will be slower because you will
    // have to iterate over more r values.
    if r > n/2 {
        return choose(n, n - r)
    }
    let mut ans = 1;
    for i in 1..r+1 {
        ans = (ans * n - r + i) / i; // who knows what this does ¯\_(ツ)_/¯
    }
    ans
}
