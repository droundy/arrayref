//! This package contains just two macros, for the taking of array
//! references to slices of anything that can be sliced.
//!
//! # Examples
//!
//! Here is a simple example of slicing and dicing a slice into array
//! references with these macros.
//!
//! I would give a real example here, but I can't figure out how to
//! make my doctest actually load the macros...
//!
//! let mut foobar = [0; 512];
//! let bar = array_ref!(foobar, 0, u16, 8); // first 8 elements
//!

#![deny(warnings)]

#[cfg(test)]
extern crate quickcheck;

/// You can use `array_ref` to generate an array reference to a subset
/// of a sliceable bit of data (which could be an array, or a slice,
/// or a Vec).
///
/// **Panics** if the slice is out of bounds.
#[macro_export]
macro_rules! array_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            unsafe fn as_array<T>(slice: &[T]) -> &[T; $len] {
                &*(slice.as_ptr() as *const [_; $len])
            }
            let slice = & $arr[$offset..$offset + $len];
            unsafe {
                as_array(slice)
            }
        }
    }}
}

/// You can use `array_refs` to generate a series of array references
/// to an input array reference.  The idea is if you want to break an
/// array into a series of contiguous and non-overlapping arrays.
#[macro_export]
macro_rules! array_refs {
    ( $arr:expr, $( $len:expr ),* ) => {{
        {
            #[inline]
            #[allow(unused_assignments)]
            unsafe fn as_arrays<T>(a: &[T; $( $len + )* 0 ]) -> ( $( &[T; $len], )* ) {
                let mut p = a.as_ptr();
                ( $( {
                    let aref = &*(p as *const [T; $len]);
                    p = p.offset($len);
                    aref
                } ),* )
            }
            let input = $arr;
            unsafe {
                as_arrays(input)
            }
        }
    }}
}


/// You can use `mut_array_refs` to generate a series of mutable array
/// references to an input mutable array reference.  The idea is if
/// you want to break an array into a series of contiguous and
/// non-overlapping arrays.
#[macro_export]
macro_rules! mut_array_refs {
    ( $arr:expr, $( $len:expr ),* ) => {{
        {
            #[inline]
            #[allow(unused_assignments)]
            unsafe fn as_arrays<T>(a: &mut [T; $( $len + )* 0 ]) -> ( $( &mut [T; $len], )* ) {
                let mut p = a.as_mut_ptr();
                ( $( {
                    let aref = &mut *(p as *mut [T; $len]);
                    p = p.offset($len);
                    aref
                } ),* )
            }
            let input = $arr;
            unsafe {
                as_arrays(input)
            }
        }
    }}
}

/// You can use `array_mut_ref` to generate a mutable array reference
/// to a subset of a sliceable bit of data (which could be an array,
/// or a slice, or a Vec).
///
/// **Panics** if the slice is out of bounds.
#[macro_export]
macro_rules! array_mut_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            unsafe fn as_array<T>(slice: &mut [T]) -> &mut [T; $len] {
                &mut *(slice.as_mut_ptr() as *mut [_; $len])
            }
            let slice = &mut $arr[$offset..$offset + $len];
            unsafe {
                as_array(slice)
            }
        }
    }}
}

#[test]
#[should_panic]
fn checks_bounds() {
    let foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bar = array_ref!(foo, 1, 11);
    bar[0];
}

#[test]
fn simple_case_works() {
    fn check(expected: [u8; 3], actual: &[u8; 3]) {
        for (e, a) in (&expected).iter().zip(actual.iter()) {
            assert_eq!(e, a)
        }
    }
    let mut foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    {
        let bar = array_ref!(foo, 2, 3);
        check([2, 3, 4], bar);
    }
    check([0, 1, 2], array_ref!(foo, 0, 3));
    fn zero2(x: &mut [u8; 2]) {
        x[0] = 0;
        x[1] = 0;
    }
    zero2(array_mut_ref!(foo, 8, 2));
    check([0, 0, 10], array_ref!(foo, 8, 3));
}


#[test]
fn check_array_ref_5() {
    fn f(data: Vec<u8>, offset: usize) -> quickcheck::TestResult {
        if data.len() < offset + 5 {
            return quickcheck::TestResult::discard();
        }
        let out = array_ref!(data, offset, 5);
        quickcheck::TestResult::from_bool(out.len() == 5)
    }
    quickcheck::quickcheck(f as fn(Vec<u8>, usize) -> quickcheck::TestResult);
}

#[test]
fn check_array_ref_out_of_bounds_5() {
    fn f(data: Vec<u8>, offset: usize) -> quickcheck::TestResult {
        if data.len() >= offset + 5 {
            return quickcheck::TestResult::discard();
        }
        quickcheck::TestResult::must_fail(move || {
            array_ref!(data, offset, 5);
        })
    }
    quickcheck::quickcheck(f as fn(Vec<u8>, usize) -> quickcheck::TestResult);
}

#[test]
fn check_array_mut_ref_7() {
    fn f(mut data: Vec<u8>, offset: usize) -> quickcheck::TestResult {
        if data.len() < offset + 7 {
            return quickcheck::TestResult::discard();
        }
        let out = array_mut_ref!(data, offset, 7);
        out[6] = 3;
        quickcheck::TestResult::from_bool(out.len() == 7)
    }
    quickcheck::quickcheck(f as fn(Vec<u8>, usize) -> quickcheck::TestResult);
}


#[test]
fn check_array_mut_ref_out_of_bounds_32() {
    fn f(mut data: Vec<u8>, offset: usize) -> quickcheck::TestResult {
        if data.len() >= offset + 32 {
            return quickcheck::TestResult::discard();
        }
        quickcheck::TestResult::must_fail(move || {
            array_mut_ref!(data, offset, 32);
        })
    }
    quickcheck::quickcheck(f as fn(Vec<u8>, usize) -> quickcheck::TestResult);
}


#[test]
fn test_5_array_refs() {
    let mut data: [usize; 128] = [0; 128];
    for i in 0..128 {
        data[i] = i;
    }
    let data = data;
    let (a,b,c,d,e) = array_refs!(&data, 1, 14, 3, 100, 10);
    assert_eq!(a.len(), 1 as usize);
    assert_eq!(b.len(), 14 as usize);
    assert_eq!(c.len(), 3 as usize);
    assert_eq!(d.len(), 100 as usize);
    assert_eq!(e.len(), 10 as usize);
    assert_eq!(a, array_ref![data, 0, 1]);
    assert_eq!(b, array_ref![data, 1, 14]);
    assert_eq!(c, array_ref![data, 15, 3]);
    assert_eq!(e, array_ref![data, 118, 10]);
}


#[test]
fn test_5_mut_xarray_refs() {
    let mut data: [usize; 128] = [0; 128];
    {
        // temporarily borrow the data to modify it.
        let (a,b,c,d,e) = mut_array_refs!(&mut data, 1, 14, 3, 100, 10);
        assert_eq!(a.len(), 1 as usize);
        assert_eq!(b.len(), 14 as usize);
        assert_eq!(c.len(), 3 as usize);
        assert_eq!(d.len(), 100 as usize);
        assert_eq!(e.len(), 10 as usize);
        *a = [1; 1];
        *b = [14; 14];
        *c = [3; 3];
        *d = [100; 100];
        *e = [10; 10];
    }
    assert_eq!(&[1;1], array_ref![data, 0, 1]);
    assert_eq!(&[14;14], array_ref![data, 1, 14]);
    assert_eq!(&[3;3], array_ref![data, 15, 3]);
    assert_eq!(&[10;10], array_ref![data, 118, 10]);
}
