// cargo expand --bin table_types

// #[table_types_macro::show_streams(R, S, T)]
// enum Table {
//     A,
//     B,
//     C,
//     D,
// }

#[table_types_macro::table(Request, S)]
enum Table {
    A(u16, u8),
    B(u16, u8),
    C(u16, u8),
    D(u16, u8),
}

pub fn main() {
    s();
}
