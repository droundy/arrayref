# arrayref

[![Build Status](https://travis-ci.org/droundy/arrayref.svg?branch=master)](https://travis-ci.org/droundy/arrayref)
[![Coverage Status](https://coveralls.io/repos/droundy/arrayref/badge.svg?branch=master&service=github)](https://coveralls.io/github/droundy/arrayref?branch=master)

[Documentation](https://droundy.github.io/arrayref)

This is a very small rust module, which contains just four macros, for
the taking of array references to slices of... sliceable things.
These macros (which are awkwardly named) should be perfectly safe, and
have seen just a tad of code review.

If you are considering providing code review, but think it might be
too time-consuming, please consider that this is only a few lines of
actual code, although I have considerably more test code.

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

For an example of how these macros can be used (albeit not a great
example, as I haven't fully cleaned up this code), see
[my rust translation of tweetnacl](https://github.com/droundy/onionsalt/blob/master/src/crypto.rs).
