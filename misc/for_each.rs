//fn main() {
//    let mut numbers = vec![1, 2, 3, 4];
//    numbers.iter_mut().for_each(|n| *n += 1);
//    println!("{:?}", numbers); // Should print [2, 3, 4, 5]
//                               //
//}
//

struct ForEachOnlyIterator<'a, T> {
    inner: &'a mut T,
}

impl<T> ForEachOnlyIterator<'_, T> {
    fn new(inner: &mut T) -> Self {
        ForEachOnlyIterator { inner }
    }

    // Define a for_each method for the custom iterator
    fn for_each<F>(&mut self, f: F)
    where
        F: FnMut(&mut T),
    {
        let x = *self.inner;
        x.iter_mut().for_each(f);
    }
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    let mut iter = ForEachOnlyIterator::new(&mut data);

    iter.for_each(|x| x +=1; println!("{}", x));

    println!("{:?}", data);

    // Uncommenting the following line will result in a compilation error:
    // iter.next();
}
