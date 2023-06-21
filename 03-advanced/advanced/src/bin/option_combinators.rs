fn main() {
    let a = None;
    dbg!(a);
    let a_is_some = a.is_some();
    dbg!(a_is_some);
    let a_is_none = a.is_none();
    dbg!(a_is_none);
    let a_mapped = a.map(|x| x + 1);
    dbg!(a_mapped);
    // NOTE .filter() returns Some<T> if true, None if false
    let a_filtered = a.filter(|x| x == &1);
    dbg!(a_filtered);
    // NOTE if a is Some<T> return a, else return Some(2)
    let a_or_else = a.or_else(|| Some(2));
    dbg!(a_or_else);
    // NOTE Returns Some<T> if Some<T>, else return 0
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}
