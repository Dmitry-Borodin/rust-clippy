#![warn(clippy::match_ref_pats)]
#![allow(clippy::equatable_if_let)]

fn ref_pats() {
    {
        let v = &Some(0);
        match v {
            &Some(v) => println!("{:?}", v),
            &None => println!("none"),
        }
        match v {
            // This doesn't trigger; we have a different pattern.
            &Some(v) => println!("some"),
            other => println!("other"),
        }
    }
    let tup = &(1, 2);
    match tup {
        &(v, 1) => println!("{}", v),
        _ => println!("none"),
    }
    // Special case: using `&` both in expr and pats.
    let w = Some(0);
    match &w {
        &Some(v) => println!("{:?}", v),
        &None => println!("none"),
    }
    // False positive: only wildcard pattern.
    let w = Some(0);
    #[allow(clippy::match_single_binding)]
    match w {
        _ => println!("none"),
    }

    let a = &Some(0);
    if let &None = a {
        println!("none");
    }

    let b = Some(0);
    if let &None = &b {
        println!("none");
    }
}

mod ice_3719 {
    macro_rules! foo_variant(
        ($idx:expr) => (Foo::get($idx).unwrap())
    );

    enum Foo {
        A,
        B,
    }

    impl Foo {
        fn get(idx: u8) -> Option<&'static Self> {
            match idx {
                0 => Some(&Foo::A),
                1 => Some(&Foo::B),
                _ => None,
            }
        }
    }

    fn ice_3719() {
        // ICE #3719
        match foo_variant!(0) {
            &Foo::A => println!("A"),
            _ => println!("Wild"),
        }
    }
}

fn main() {}
