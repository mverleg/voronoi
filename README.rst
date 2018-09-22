
Voronoi
===============================

This little project:

* Chooses a bunch of random points on an image.
* Assigns each pixel to one of the random points (Voronoi).
*

It's just a little challenge:

* Try to get good performance, by using algorithms with good complexity and by avoiding inefficiencies. No advanced math of GPUs, though.
* Make it all as type-safe as possible.

Examples
-------------------------------

Not ready yet...

Performance
-------------------------------

* There are a lot of wrapper types for safety, but these shouldn't cost anything in Rust.
* Try to avoid heap allocations. The image itself, the list of centers and the pixel mapping should be heap-allocated though, their size is large and unknown at compile-time.
* Try to avoid dynamic dispatch completely.
* Avoid unnecessary math, e.g. L1/L2/L3 norm values have the same total order without the square/cubic root.
* Avoid unnecessary allocations, i.e. keep recycling one vector per thread to store nearest points.

Todo: simd, cache locality, contiguous memory, parallelism, branch prediction, 

Types
-------------------------------

There are several considerations:

* There are pixels indexed in two dimensions by positive integers. We should not mix these dimensions or compare x and y for different points.
* There is iteration over bounded regions of the image. So a total ordering as well as addition and subtraction are needed.
* The sum of pixel positions has no meaning, but they may be summed to compute a midpoint position. But they can be subtracted, which gives a distance.
* Distances are not real numbers and do not care about direction.
* For polymorphism reasons, different norms should return identical or compatible types, even though their physical units are different, since `sqrt`s are skipped.

Difficulties:

* Some operations cannot be overloaded generically but have to be re-done for every concrete type because of orphan rules. Macros help a little here.
* It is necessary to expose the `usize` data for e.g. generating random numbers or building a `Vec`.
* There is substantially more code to facilitate types and operations on them than there is 'business' logic.

How to use
-------------------------------

* Compile: `RUSTFLAGS="-C target-cpu=native" cargo build --release`
* Decrease size: `strip target/release/voronoi`
* Run: `target/release/voronoi resources/imgs/parrots.png --show`

