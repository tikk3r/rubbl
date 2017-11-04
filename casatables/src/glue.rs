/* automatically generated by rust-bindgen */

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _GlueDataType {
    TpBool = 0,
    TpChar = 1,
    TpUChar = 2,
    TpShort = 3,
    TpUShort = 4,
    TpInt = 5,
    TpUInt = 6,
    TpFloat = 7,
    TpDouble = 8,
    TpComplex = 9,
    TpDComplex = 10,
    TpString = 11,
    TpTable = 12,
    TpArrayBool = 13,
    TpArrayChar = 14,
    TpArrayUChar = 15,
    TpArrayShort = 16,
    TpArrayUShort = 17,
    TpArrayInt = 18,
    TpArrayUInt = 19,
    TpArrayFloat = 20,
    TpArrayDouble = 21,
    TpArrayComplex = 22,
    TpArrayDComplex = 23,
    TpArrayString = 24,
    TpRecord = 25,
    TpOther = 26,
    TpQuantity = 27,
    TpArrayQuantity = 28,
    TpInt64 = 29,
    TpArrayInt64 = 30,
}
pub use self::_GlueDataType as GlueDataType;
/// <div rustbindgen nocopy></div>
#[repr(C)]
#[derive(Debug)]
pub struct _GlueString {
    pub a: *mut ::std::os::raw::c_void,
    pub b: *mut ::std::os::raw::c_void,
    pub c: *mut ::std::os::raw::c_void,
    pub d: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__GlueString() {
    assert_eq!(::std::mem::size_of::<_GlueString>() , 32usize , concat ! (
               "Size of: " , stringify ! ( _GlueString ) ));
    assert_eq! (::std::mem::align_of::<_GlueString>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( _GlueString ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _GlueString ) ) . a as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _GlueString ) , "::" ,
                stringify ! ( a ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _GlueString ) ) . b as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( _GlueString ) , "::" ,
                stringify ! ( b ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _GlueString ) ) . c as * const _ as usize
                } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( _GlueString ) , "::" ,
                stringify ! ( c ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _GlueString ) ) . d as * const _ as usize
                } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( _GlueString ) , "::" ,
                stringify ! ( d ) ));
}
pub type GlueString = _GlueString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GlueTable {
    _unused: [u8; 0],
}
pub type GlueTable = _GlueTable;
#[repr(C)]
#[derive(Copy)]
pub struct _ExcInfo {
    pub message: [::std::os::raw::c_char; 512usize],
}
#[test]
fn bindgen_test_layout__ExcInfo() {
    assert_eq!(::std::mem::size_of::<_ExcInfo>() , 512usize , concat ! (
               "Size of: " , stringify ! ( _ExcInfo ) ));
    assert_eq! (::std::mem::align_of::<_ExcInfo>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( _ExcInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _ExcInfo ) ) . message as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _ExcInfo ) , "::" ,
                stringify ! ( message ) ));
}
impl Clone for _ExcInfo {
    fn clone(&self) -> Self { *self }
}
pub type ExcInfo = _ExcInfo;
extern "C" {
    pub fn string_check_size() -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn string_init(str: *mut GlueString,
                       data: *const ::std::os::raw::c_void,
                       n_bytes: ::std::os::raw::c_ulong);
}
extern "C" {
    pub fn string_deinit(str: *mut GlueString);
}
extern "C" {
    pub fn table_alloc_and_open(path: *const GlueString, exc: *mut ExcInfo)
     -> *mut GlueTable;
}
extern "C" {
    pub fn table_close_and_free(table: *mut GlueTable, exc: *mut ExcInfo);
}
extern "C" {
    pub fn table_n_rows(table: *const GlueTable) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn table_deep_copy_no_rows(table: *const GlueTable,
                                   dest_path: *const GlueString,
                                   exc: *mut ExcInfo)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_column_info(table: *const GlueTable,
                                 col_name: *const GlueString,
                                 n_rows: *mut ::std::os::raw::c_ulong,
                                 data_type: *mut GlueDataType,
                                 is_scalar: *mut ::std::os::raw::c_int,
                                 is_fixed_shape: *mut ::std::os::raw::c_int,
                                 n_dim: *mut ::std::os::raw::c_uint,
                                 dims: *mut ::std::os::raw::c_ulong,
                                 exc: *mut ExcInfo) -> ::std::os::raw::c_int;
}
