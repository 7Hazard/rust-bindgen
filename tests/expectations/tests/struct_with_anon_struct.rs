/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct foo {
    pub bar: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_1>() , 8usize , concat !
               ( "Size of: " , stringify ! ( foo__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<foo__bindgen_ty_1>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( foo__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . a as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo__bindgen_ty_1 ) ,
                "::" , stringify ! ( a ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . b as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( foo__bindgen_ty_1 ) ,
                "::" , stringify ! ( b ) ));
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 8usize , concat ! (
               "Size of: " , stringify ! ( foo ) ));
    assert_eq! (::std::mem::align_of::<foo>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( foo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . bar as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo ) , "::" ,
                stringify ! ( bar ) ));
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}
