/**
This macro is used to create a macro that itself derives a set of traits.

# Example

```
derive_alias! {
    derive_stuff => Eq, PartialEq, Ord, PartialOrd
}

derive_stuff! { struct Foo(i32); }
```
*/
#[macro_export]
macro_rules! derive_alias {
    ($name:ident => $($derive:ident),*) => {
        macro_rules! $name {
            ($i:item) => {
                #[derive($($derive),*)]
                $i
            }
        }
    }
}

/**
This macro is used to create multiple macros that themselves derive a set of traits.

# Example

```
derive_alias! {
    derive_stuff => Eq, PartialEq, Ord, PartialOrd;
    derive_stuff2 => Clone, Copy, Debug;
    derive_stuff3 => Hash, Sequence, MyEnumIter;
}

derive_stuff! { struct Foo(i32); }
derive_stuff2! { struct Bar(i32); }
derive_stuff3! {
    struct Baz {
        a: i32,
        b: i32,
    }
}
```
*/
#[macro_export]
macro_rules! derive_aliases {
    ($($name:ident => $($derive:ident),*;)*) => {
        $(
            derive_alias! {
                $name => $($derive),*
            }
        )*
    }
}

derive_aliases! {}
