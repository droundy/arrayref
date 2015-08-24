# arrayref

This is a very small rust module, which contains just two macros, for
the taking of array references to slices of... sliceable things.
These two macros (which are awkwardly named) should be perfectly safe,
but this is my first time writing unsafe code, so I will **greatly**
appreciate any code review.

If you are considering providing code review, but think it might be
too time-consuming, please consider that this is only 21 lines of
actual code, although I have another 80 or so lines of test code.

The basic idea is that when we know at compile time the size of a
slice we want, we should be able to take an array reference instead of
a slice, which then will allow us to statically guarantee that the
input to functions is of the correct size (when it is statically known
what the size should be.

The motivating example is in encryption code, in which we often deal
with pointers to small chunks of bytes that are of fixed size
(e.g. keys, nonces, hashes, or authentication blocks).  It would be
lovely to express these constraints in our function signatures, but
that has the side effect of requiring a copy of the containing slice
to be made, in the common case where the information was embedded in a
larger `&[u8]`.

