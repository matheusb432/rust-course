// ? Vec Macro that can be separated by `;`
macro_rules! c_vec {
    (
        $( $el: expr );*
        $(;)?
    ) => {{
        let mut v = Vec::new();
        $(
            v.push($el);
        )*
        v
    }}
}

macro_rules! init_mut_vec {
    (
        $name: ident => $( $el: expr );*
        $(;)?
    ) => {
        let mut $name = c_vec!($($el);*);
    };
}

fn main() {
    // ? Can invoke the macro with `[]` or `()` or even `{}`
    let new_vec = c_vec!["he"; "llo"; " world!"];
    let newest_vec = c_vec!("trailing"; "separator!";);
    let cooler_vec = c_vec! { 1; 2; 3 };
    init_mut_vec!(another_vec => 1.0; 2.5);
    another_vec.push(3.0);

    assert_eq!(new_vec, vec!["he", "llo", " world!"]);
    assert_eq!(newest_vec, vec!["trailing", "separator!"]);
    assert_eq!(cooler_vec, vec![1, 2, 3]);
    assert_eq!(another_vec, vec![1.0, 2.5, 3.0]);

    dbg!(new_vec);
    dbg!(newest_vec);
    dbg!(cooler_vec);
    dbg!(another_vec);
}
