//! These are wrappers for combinations of `slice::split_at` or
//! `slice::split_at_mut` with `array_ref` and `array_mut_ref`
//! respectively, that simplify handling a variety of references
//! into a slice.
//! 
//! In essence, they provide a borrow checker friendly approach to
//! `{ (r,heap) = heap.split_at_mut(len); array_mut_ref![r,0,len] }`


/// Reserve an initial segment of a slice as a slice.
///
/// Returns an slice occupying an initial segment of length `len`
/// of a slice presented as a `&mut &[T]` while replacing the inner
/// `&[T]` with the remainder.  
/// 
/// This is an ergonomic wrapper on `slice::split_at`.  It acts like
/// the pseudocode `{ (r,heap) = heap.split_at(len); r }` but keeps
/// the borrow checker happy. 
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// extern crate arrayref;
/// use arrayref::reserve;
/// // ...
/// # fn main() {
/// let mut data : &[u16] = &[0,1,2,3,4,5,6,7,8,9];
/// let head = reserve(&mut data,5);
/// for (i,j) in data.iter().zip(head) {
///    assert_eq!(*i,*j+5);
/// }
/// # }
/// ```
pub fn reserve<'heap, T>(heap: &mut &'heap [T], len: usize) -> &'heap [T] {
    let tmp: &'heap [T] = ::core::mem::replace(&mut *heap, &[]);
    let (reserved, tmp) = tmp.split_at(len);
    *heap = tmp;
    reserved
}

/// Reserve an initial segment of a slice as a fixed length array.
///
/// Returns a reference to a fixed length array occupying an initial
/// segment of a slice presented as a `&mut &[T]` while replacing the
/// inner `&[T]` with the remainder.  
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// #[macro_use]
/// extern crate arrayref;
///
/// struct Refs<'a> {
///     head: &'a [u16; 5],
///     tail: &'a [u16],
/// }
/// // ...
/// # fn main() {
/// let mut data : &[u16] = &[0,1,2,3,4,4,3,2,1,0];
/// let refs = Refs {
///     head: reserve_fixed!(&mut data,5),
///     tail: ::arrayref::reserve(&mut data,5),
/// };
/// assert!(data.is_empty());
/// for (i,j) in refs.head.iter().zip(refs.tail) {
///    assert_eq!(*i + *j, 4);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! reserve_fixed { ($heap:expr, $len:expr) => {
    array_ref![::arrayref::reserve($heap,$len),0,$len]
} }


/// Reserve a trailing segment of a slice as a slice.
///
/// Returns an slice occupying a trailing segment of length `len`
/// of a slice presented as a `&mut &[T]` while replacing the inner
/// `&[T]` with the remainder.
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// extern crate arrayref;
/// use arrayref::reserve_tail;
/// // ...
/// # fn main() {
/// let mut data : &[u16] = &[0,1,2,3,4,5,6,7,8,9];
/// let tail = reserve_tail(&mut data,5);
/// for (i,j) in data.iter().zip(tail) {
///    assert_eq!(*i+5,*j);
/// }
/// # }
/// ```
pub fn reserve_tail<'heap, T>(heap: &mut &'heap [T], len: usize) -> &'heap [T] {
    let tmp: &'heap [T] = ::core::mem::replace(&mut *heap, &[]);
    let l = tmp.len() - len;
    let (tmp, reserved) = tmp.split_at(l);
    *heap = tmp;
    reserved
}

/// Reserve a trailing segment of a slice as a fixed length array.
///
/// Returns a reference to a fixed length array occupying a trailing
/// segment of a slice presented as a `&mut &[T]` while replacing the
/// inner `&[T]` with the remainder.  
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// #[macro_use]
/// extern crate arrayref;
///
/// struct Refs<'a> {
///     head: &'a [i32],
///     tail: &'a [i32; 5],
/// }
/// // ...
/// # fn main() {
/// let mut data : &[i32] = &[0,1,2,3,4,4,3,2,1,0];
/// let refs = Refs {
///     tail: reserve_tail_fixed!(&mut data,5),
///     head: ::arrayref::reserve_tail(&mut data,5),
/// };
/// assert!(data.is_empty());
/// for (i,j) in refs.head.iter().zip(refs.tail) {
///    assert_eq!(*i + *j, 4);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! reserve_tail_fixed { ($heap:expr, $len:expr) => {
    array_ref![::arrayref::reserve($heap,$len),0,$len]
} }


/// Reserve an initial segment of a mutable slice as a mutable slice.
///
/// Returns a mutable slice occupying an initial segment of length
/// `len` of a mutable slice presented as a `&mut &mut [T]` while
/// replacing the inner `&[T]` with the remainder.  
/// 
/// This is an ergonomic wrapper on `slice::split_at_mut`.  It acts
/// like the pseudocode `{ (r,heap) = heap.split_at_mut(len); r }`
/// but apeases the borrow checker. 
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// extern crate arrayref;
/// use arrayref::reserve_mut;
/// // ...
/// # fn main() {
/// let mut data : &mut [isize] = &mut [0,1,2,3,4,0,6,7,8,9];
/// let head = reserve_mut(&mut data,5);
/// for i in head.iter_mut().skip(1) { *i+=5; }
/// assert_eq!(head,data);
/// # }
/// ```
// Originally by nox. See http://stackoverflow.com/a/42162816/667457
pub fn reserve_mut<'heap, T>(heap: &mut &'heap mut [T], len: usize) -> &'heap mut [T] {
    let tmp: &'heap mut [T] = ::core::mem::replace(&mut *heap, &mut []);
    let (reserved, tmp) = tmp.split_at_mut(len);
    *heap = tmp;
    reserved
}

/// Reserve an initial segment of a mutable slice as a mutably borrowed
/// fixed length array.
///
/// Returns a mutable reference to a fixed length array occupying an
/// initial segment of a slice presented as a `&mut &mut [T]` while
/// replacing the inner `&mut [T]` with the remainder.  
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// #[macro_use]
/// extern crate arrayref;
///
/// struct Muts<'a> {
///     head: &'a mut [u16; 5],
///     tail: &'a mut [u16],
/// }
/// // ...
/// # fn main() {
/// let mut data : &mut [u16] = &mut [0,1,2,3,4,0,6,7,8,9];
/// let muts = Muts {
///     head: reserve_fixed_mut!(&mut data,5),
///     tail: ::arrayref::reserve_mut(&mut data,5),
/// };
/// assert!(data.is_empty());
/// for i in muts.head.iter_mut().skip(1) { *i+=5; }
/// assert_eq!(muts.head,muts.tail);
/// # }
/// ```
#[macro_export]
macro_rules! reserve_fixed_mut { ($heap:expr, $len:expr) => {
    array_mut_ref![::arrayref::reserve_mut($heap,$len),0,$len]
} }


/// Reserve an trailing segment of a mutable slice as a mutable slice.
///
/// Returns a mutable slice occupying an trailing segment of length
/// `len` of a mutable slice presented as a `&mut &mut [T]` while
/// replacing the inner `&[T]` with the remainder.  
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// extern crate arrayref;
/// use arrayref::reserve_tail_mut;
/// // ...
/// # fn main() {
/// let mut data : &mut [u16] = &mut [0,1,2,3,4,0,6,7,8,9];
/// let tail = reserve_tail_mut(&mut data,5);
/// for i in tail.iter_mut().skip(1) { *i-=5; }
/// assert_eq!(tail,data);
/// # }
/// ```
pub fn reserve_tail_mut<'heap, T>(heap: &mut &'heap mut [T], len: usize) -> &'heap mut [T] {
    let tmp: &'heap mut [T] = ::core::mem::replace(&mut *heap, &mut []);
    let l = tmp.len() - len;
    let (tmp, reserved) = tmp.split_at_mut(l);
    *heap = tmp;
    reserved
}

/// Reserve an trailing segment of a mutable slice as a mutably borrowed
/// fixed length array.
///
/// Returns a mutable reference to a fixed length array occupying an
/// trailing segment of a slice presented as a `&mut &mut [T]` while
/// replacing the inner `&mut [T]` with the remainder.  
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// #[macro_use]
/// extern crate arrayref;
///
/// struct Muts<'a> {
///     tail: &'a mut [u16; 5],
///     head: &'a mut [u16],
/// }
/// // ...
/// # fn main() {
/// let mut data : &mut [u16] = &mut [0,1,2,3,4,0,6,7,8,9];
/// let muts = Muts {
///     tail: reserve_tail_fixed_mut!(&mut data,5),
///     head: ::arrayref::reserve_tail_mut(&mut data,5),
/// };
/// assert!(data.is_empty());
/// for i in muts.tail.iter_mut().skip(1) { *i-=5; }
/// assert_eq!(muts.head,muts.tail);
/// # }
/// ```
#[macro_export]
macro_rules! reserve_tail_fixed_mut { ($heap:expr, $len:expr) => {
    array_mut_ref![::arrayref::reserve_tail_mut($heap,$len),0,$len]
} }


/*

#[cfg(test)]
mod test {

extern crate quickcheck;



} // mod test

*/

