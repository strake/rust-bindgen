/* automatically generated by rust-bindgen */


#![feature(const_fn)]
#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug)]
pub struct Struct_Foo<T, U> {
    pub m_member: T,
    pub m_member_ptr: *mut T,
    pub m_member_arr: [T; 1usize],
    pub _phantom0: ::std::marker::PhantomData<U>,
}
#[repr(C)]
#[derive(Debug)]
pub struct Struct_D<T> {
    pub m_foo: Struct_Foo<::std::os::raw::c_int, ::std::os::raw::c_int>,
    pub _phantom0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug)]
pub struct Struct_D_U<T, Z> {
    pub m_nested_foo: Struct_Foo<::std::os::raw::c_int,
                                 ::std::os::raw::c_int>,
    pub m_baz: Z,
    pub _phantom0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_Rooted<T> {
    pub prev: *mut T,
    pub next: *mut Struct_Rooted<*mut ::std::os::raw::c_void>,
    pub ptr: T,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_RootedContainer {
    pub root: Struct_Rooted<*mut ::std::os::raw::c_void>,
}
impl ::std::clone::Clone for Struct_RootedContainer {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_RootedContainer() {
    assert_eq!(::std::mem::size_of::<Struct_RootedContainer>() , 24usize);
    assert_eq!(::std::mem::align_of::<Struct_RootedContainer>() , 8usize);
}
pub type WithDtorIntFwd = Struct_WithDtor<::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug)]
pub struct Struct_WithDtor<T> {
    pub member: T,
}
#[repr(C)]
#[derive(Debug)]
pub struct Struct_PODButContainsDtor {
    pub member: WithDtorIntFwd,
}
#[test]
fn bindgen_test_layout_Struct_PODButContainsDtor() {
    assert_eq!(::std::mem::size_of::<Struct_PODButContainsDtor>() , 4usize);
    assert_eq!(::std::mem::align_of::<Struct_PODButContainsDtor>() , 4usize);
}
#[repr(C)]
pub struct Opaque;
#[repr(C)]
pub struct Struct_POD {
    pub opaque_member: u32,
}
#[test]
fn bindgen_test_layout_Struct_POD() {
    assert_eq!(::std::mem::size_of::<Struct_POD>() , 4usize);
    assert_eq!(::std::mem::align_of::<Struct_POD>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_NestedBase<T, U> {
    pub buff: *mut T,
    pub _phantom0: ::std::marker::PhantomData<U>,
}
/**
 * <div rustbindgen replaces="NestedReplaced"></div>
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_NestedReplaced<T> {
    pub buff: *mut T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_NestedContainer<T> {
    pub c: T,
    pub nested: Struct_NestedReplaced<T>,
    pub inc: Struct_Incomplete<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_Incomplete<T> {
    pub d: T,
}
extern "C" {
    #[link_name = "_Z3bar3FooIiiE"]
    pub fn bar(foo: Struct_Foo<::std::os::raw::c_int, ::std::os::raw::c_int>);
}
