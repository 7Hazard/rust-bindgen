/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


pub const foo_THIS: foo = 0;
pub const foo_SHOULD_BE: foo = 1;
pub const foo_A_CONSTANT: foo = 2;
pub type foo = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct bar {
    pub this_should_work: foo,
}
#[test]
fn bindgen_test_layout_bar() {
    assert_eq!(::std::mem::size_of::<bar>() , 4usize , concat ! (
               "Size of: " , stringify ! ( bar ) ));
    assert_eq! (::std::mem::align_of::<bar>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( bar ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const bar ) ) . this_should_work as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( bar ) , "::" ,
                stringify ! ( this_should_work ) ));
}
impl Clone for bar {
    fn clone(&self) -> Self { *self }
}
impl Default for bar {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
