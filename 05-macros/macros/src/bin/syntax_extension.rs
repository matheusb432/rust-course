macro_rules! iterslice {
    // * iterable[k]
    // * iterable[3] -> retrieve item at index 3
    ($iterable:ident [ $index:literal ]) => {{
        let mut iterable = $iterable.iter();
        iterable.nth($index).expect("index out of bounds")
    }};
    // * iterable[start:]
    // * iterable[3:] -> skip 3, until the end
    ($iterable: ident [ $index:literal :]) => {{
        let iterable = $iterable.iter();
        iterable.skip($index)
    }};
    // * iterable[:end-1]
    // * iterable[:5] -> take everything up to (but not including) index 5
    ($iterable: ident [ : $index:literal ]) => {{
        let iterable = $iterable.iter();
        iterable.take($index)
    }};

    // * iterable[start:end-1]
    // * iterable[1:3] -> items at index 1 and 2
    ($iterable: ident [$start:literal : $end:literal]) => {{
        let iterable = $iterable.iter();
        if $start > $end {
            panic!("start index > end index")
        }

        if $start == $end {
            iterable.skip($start).take(1)
        } else {
            iterable.skip($start).take($end - $start)
        }
    }};
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let new_iter: Vec<_> = iterslice!(numbers[1:5]).collect();
    dbg!(new_iter);
}

#[cfg(test)]
mod test {

    #[test]
    fn iter_range() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        // * iterable[start:end-1]
        // * iterable[1:3] -> items at index 1 and 2
        let new_iter = iterslice!(numbers[1:5]);
        let expected = vec![2, 3, 4, 5];

        for (expect, actual) in new_iter.zip(expected.iter()) {
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn iter_until_n() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        // iterable[:end-1]
        // iterable[:5] -> take everything up to (but not including) index 5
        let new_iter = iterslice!(numbers[:4]);
        let expected = vec![1, 2, 3, 4];

        for (expect, actual) in new_iter.zip(expected.iter()) {
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn iter_from_n_until_end() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        // iterable[start:]
        // iterable[3:] -> skip 3, until the end
        let new_iter = iterslice!(numbers[2:]);
        let expected = vec![3, 4, 5, 6];

        for (expect, actual) in new_iter.zip(expected.iter()) {
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn get_index() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        // iterable[k]
        // iterable[3] -> retrieve item at index 3
        let value = iterslice!(numbers[4]);
        let expected = &5;
        assert_eq!(expected, value);
    }
}
