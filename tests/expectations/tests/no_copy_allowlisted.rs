#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct NoCopy {
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NoCopy() {
    assert_eq!(
        ::std::mem::size_of::<NoCopy>(),
        4usize,
        concat!("Size of: ", stringify!(NoCopy))
    );
    assert_eq!(
        ::std::mem::align_of::<NoCopy>(),
        4usize,
        concat!("Alignment of ", stringify!(NoCopy))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<NoCopy>() };
            let struct_ptr = &struct_instance as *const NoCopy;
            let field_ptr = std::ptr::addr_of!(struct_instance.i);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(NoCopy), "::", stringify!(i))
    );
}