// Yes, yes, we know. It's an exercise, compiler, we want it that way!
#[allow(unused_mut)]

fn main() {
    // 1. Uncomment the code below. Create a closure that returns the square of an integer (the
    // number multiplied by itself), and assign the closure to the "square" variable. Then run the
    // code and make sure it works.

    let square = |x| x * x;
    println!("5 squared is {}", square(5));

    // 2. Uncomment the code below.  Finish the .map() iterator adaptor call by passing it a closure
    // which takes a tuple of two integers as a parameter, and returns a tuple with the first
    // integer incremented by 1, and the second integer left alone.  For example, if given the input
    // (0, 1), it should return (1, 1). Run the code and make sure it works.

    let _modified_pairs: Vec<(i32, i32)> = vec![(0, 1), (2, 3), (4, 5)]
        .into_iter()
        .map(|t: (i32, i32)| (t.0 + 1, t.1))
        .collect();

    _modified_pairs
        .iter()
        .for_each(|t: &(i32, i32)| println!("{:?}", t));
    // 3. Uncomment the code below. There is a mutable vector named `numbers`. Use an iterator over
    // mutable references to multiply each of the values in `numbers` by 3.
    // Hint 1: You'll need .iter_mut() -- bonus points if you use the shorter, syntactic sugar form!
    // Hint 2: `x` will be a mutable reference, so remember to dereference it to use it

    let mut numbers = vec![1, 2, 3, 4];
    for num in &mut numbers {
        *num *= 3;
    }
    println!("{:?}", numbers); // should print [3, 6, 9, 12]

    // 4. Uncomment the code below.  Take the vector of words and
    // - Convert the vector into an iterator with .into_iter()
    // - Use .filter() to remove any word that contains the letter "h" -- use .contains()
    // - Use .map() to convert all the words to uppercase -- use .to_uppercase()
    // - Use .collect() to put the transformed words back into a vector
    //
    // Hint: .to_uppercase() is a method on `str` which returns a String

    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    let transformed: Vec<_> = words
        .into_iter()
        .filter(|n| !n.contains('h'))
        .map(|word| word.to_uppercase())
        .collect();
    // do the stuff here
    println!("Transformed: {:?}", transformed);

    // Challenge:
    //
    // - Rewrite the code in #2 as a for loop\
    let mut _modified_pairs_due: Vec<(i32, i32)> = vec![(0, 1), (2, 3), (4, 5)];
    for vecs in &mut _modified_pairs_due {
        vecs.0 += 1;
    }
    println!("Modified: {:?}", _modified_pairs_due);
    // - Rewrite the code in #3 in functional style (without a for loop).  Hint: There are multiple
    // ways to accomplish this, but they all end with an iterator consumer.

    let mut numbers_two = vec![1, 2, 3, 4];
    let modified: Vec<i32> = numbers_two.into_iter().map(|x| x * 3).collect();
    println!("Whatsup Beijing {:?}", modified)
}
