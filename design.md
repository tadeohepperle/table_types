What is a table type?

```rs
struct STRUCT_TY {
    x: X
    y: Y
}

let s: STRUCT_TY
s.x = X
s.y = Y

enum ENUM_TY {
    A    A_TY
    B    B_TY
}

let e: ENUM_TY

match e:
e::A  = A_TY
e::B  = B_TY

table! { TABLETY
         R         S        T
    A    AR_TY     AS_TY    AT_TY
    B    BR_TY     BS_TY    BT_TY
}

match t:

TABLE_TY::A  = {r:   AR_TY   s:  AS_TY   t:   AT_TY}
TABLE_TY::B  = {r:   BR_TY   s:  BS_TY   t:   BT_TY}

t.r  = AR_TY | BR_TY
t.s  = AS_TY | BS_TY

match t.s:   TABLE_TY::SA  =

\\


Table::E::A

Table::R::



algebraic data types:


exponential types
```
