pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// brings into scope only the 'of' module
use a::series::of;

fn main() {
    // the verbose way is the same as
    a::series::of::nested_modules();

    // -> this
    of::nested_modules();
}
