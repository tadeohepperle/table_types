# Simple Tabular dependant type enums

work in progress...

Sometimes just having structs and enums is just not enough to keep certain invariants afloat during compile time.
This is why this crate introduces a `Table` type that is basically a table of dependant enums.

This is a struct:

```rs
struct STRUCT_TY {
    x: X
    y: Y
}

let s: STRUCT_TY
s.x = X
s.y = Y
```

This is an enum:

```rs
enum ENUM_TY {
    A    A_TY
    B    B_TY
}

let e: ENUM_TY
match e:
e::A  = A_TY
e::B  = B_TY
```

And this is a table:

```rs
table! { TABLETY
         R         S        T
    A    AR_TY     AS_TY    AT_TY
    B    BR_TY     BS_TY    BT_TY
}
```

more coming soon...
