use std::collections::HashMap;

fn main() {
    struct Friends {
        names: Vec<String>,
    }

    // NOTE Move iterator
    // ? Implementation that will move the Friends struct
    impl IntoIterator for Friends {
        type Item = String;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        // ? Will return an iterator over the `names` vector
        fn into_iter(self) -> Self::IntoIter {
            self.names.into_iter()
        }
    }

    let friends = Friends {
        names: vec!["Albert".to_owned(), "Sara".to_owned()],
    };

    print!("\nMove iterator:\t");
    for f in friends {
        print!("{}\t", f);
    }

    // NOTE Borrow iterator
    // ? Implementation that borrows the Friends struct using lifetimes
    impl<'a> IntoIterator for &'a Friends {
        type Item = &'a String;
        type IntoIter = std::slice::Iter<'a, String>;

        fn into_iter(self) -> Self::IntoIter {
            self.names.iter()
        }
    }

    let friends = Friends {
        names: vec!["Albert".to_owned(), "Sara".to_owned()],
    };

    print!("\nBorrow iterator:\t");
    for f in &friends {
        print!("{}\t", f);
    }

    // NOTE Mutable borrow iterator
    impl<'a> IntoIterator for &'a mut Friends {
        type Item = &'a mut String;
        type IntoIter = std::slice::IterMut<'a, String>;

        fn into_iter(self) -> Self::IntoIter {
            self.names.iter_mut()
        }
    }
    let mut friends = Friends {
        names: vec!["Albert".to_owned(), "Sara".to_owned()],
    };
    print!("\nMutable borrow iterator:\t");
    for f in &mut friends {
        // ? Will set all names to "Friend"
        *f = "Friend".to_owned();
        print!("{}\t", f);
    }

    #[derive(Debug, Hash, Eq, PartialEq)]
    enum Fruit {
        Apple,
        Orange,
        Banana,
    }

    struct FruitStand {
        fruit: HashMap<Fruit, u32>,
    }

    // NOTE Implementing an IntoIterator for a HashMap field
    impl IntoIterator for FruitStand {
        type Item = (Fruit, u32);
        type IntoIter = std::collections::hash_map::IntoIter<Fruit, u32>;

        fn into_iter(self) -> Self::IntoIter {
            self.fruit.into_iter()
        }
    }

    impl<'a> IntoIterator for &'a FruitStand {
        type Item = (&'a Fruit, &'a u32);
        type IntoIter = std::collections::hash_map::Iter<'a, Fruit, u32>;

        fn into_iter(self) -> Self::IntoIter {
            self.fruit.iter()
        }
    }

    impl<'a> IntoIterator for &'a mut FruitStand {
        // ? The key `Fruit` doesn't need to be borrowed as mutable since they should not change
        type Item = (&'a Fruit, &'a mut u32);
        type IntoIter = std::collections::hash_map::IterMut<'a, Fruit, u32>;

        fn into_iter(self) -> Self::IntoIter {
            self.fruit.iter_mut()
        }
    }

    let mut fruit = HashMap::new();
    fruit.insert(Fruit::Banana, 5);
    fruit.insert(Fruit::Orange, 2);

    println!("\n\n&mut fruit:");
    for (k, v) in &mut fruit {
        println!("{k:?}: {v}");
        *v += 4;
    }

    println!("&fruit:");
    // ? into_iter is called implicitly, desugars to (&fruit).into_iter()
    for (k, v) in &fruit {
        println!("{k:?}: {v}");
    }

    println!("fruit:");
    for (k, v) in fruit {
        println!("{k:?}: {v}");
    }
}
