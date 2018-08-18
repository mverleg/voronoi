
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

...

Types
-------------------------------

There are several considerations:

* There are pixels indexed in two dimensions by positive integers. We should not mix these dimensions or compare x and y for different points.
* There is iteration over bounded regions of the image. So a total ordering as well as addition and subtraction are needed.
* It does not make sense to add pixel positions together, or coordinates of pixel positions. But they can be subtracted, which gives a distance.
* Distances are not real numbers and do not care about direction.
* For polymorphism reasons, different norms should return identical or compatible types, even though their physical units are different, since `sqrt`s are skipped.


