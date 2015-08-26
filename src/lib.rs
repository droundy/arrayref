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
/// or a Vec).  The arguments are a bit clumsy
#[macro_export]
macro_rules! array_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            unsafe fn as_array<T>(slice: &[T]) -> &[T; $len] {
                &*(slice.as_ptr() as *const [_; $len])
            }
            let a: usize = $offset;
            let l: usize = $len;
            unsafe {
                as_array(&$arr[a..a.saturating_add(l)])
            }
        }
    }}
}

/// You can use `array_mut_ref` to generate a mutable array reference
/// to a subset of a sliceable bit of data (which could be an array,
/// or a slice, or a Vec).
#[macro_export]
macro_rules! array_mut_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            unsafe fn as_array<T>(slice: &mut[T]) -> &mut[T; $len] {
                &mut *(slice.as_mut_ptr() as *mut [_; $len])
            }
            let a: usize = $offset;
            let l: usize = $len;
            unsafe {
                as_array(&mut $arr[a..a.saturating_add(l)])
            }
        }
    }}
}

#[test]
#[should_panic]
fn checks_bounds() {
    let foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bar = array_ref!(foo, 1, 11);
    println!("{}", bar[0]);
}

#[test]
fn simple_case_works() {
    let mut foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    fn pr3(x: &[u8; 3]) {
        println!("[{} {} {}]", x[0], x[1], x[2]);
    }
    {
        let bar = array_ref!(foo, 2, 3);
        println!("{}", bar.len());
        pr3(bar);
    }
    pr3(array_ref!(foo, 0, 3));
    fn zero2(x: &mut [u8; 2]) {
        x[0] = 0;
        x[1] = 0;
    }
    zero2(array_mut_ref!(foo, 8, 2));
    pr3(array_ref!(foo, 8, 3));
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
