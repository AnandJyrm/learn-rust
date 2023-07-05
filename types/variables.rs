fn main() {
    let _annotated_bool: bool = true;

    let _annotated_int: i32 = 42;
    let _suffix_annotated_int = 42i32;

    let _default_int = 7;

    let mut _inferred_int = 12;
    _inferred_int = 4294967296i64;

    let mut _shadowed_mut = 12;
    let _shadowed_mut = true;
}
