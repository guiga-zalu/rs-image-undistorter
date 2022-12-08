# Image Undistorter

## Reason

There are apps like "CamScanner", "Microsoft Lens", "Open Scanner" etc.  
With these, one can take a photo containing a document, and them the app will identify its borders and undistort it into a rectangle.

I want to build my own.

## Implemented Concepts

### Border recognition

As in the begginings, these apps didn't implement this, but ratter depended on user input to recognize the borders, and then processing the image, I plan on doing the same (or similar): the user shall input (somehow) points that belong to a specific border (top, left, right or bottom).

The more the number of points, the better.

### Preparation to remapping

In the processing-space, the coordenates of the image are remapped from

$$\left(\left[0; \mathrm{width}\right[_\mathbb{N}\times\left[0; \mathrm{height}\right[_\mathbb{N}\right)\to\left[0; 1\right[_\mathbb{R}^2$$

Then, with the points, for each border a polynomial is calculated, to interpolate the border. I used simple Lagrange ones.  
The polynomials are then _lerp_-ed to create a remapper:

$$\mathrm{distorted\, space}\to\mathrm{final\, space}$$

### Remapping

Thus, with this remapper, it's possible to create the new image, by processing every pixel from the original one, discarding those which would span outside of the destination, and \[somewhow\] fix the missing pixels. \[Maybe with some oversampling?\]

A better solution would try to reproduce the OpenCV approach: calculate the transformation matrix, invert it and them use it to search every pixel for the destination image, from the original one.  
But there is a problem:

I don't know how to invert this matrix.

Inverting polynomials on one variable is one thing. But inverting a \[very\] _lerp_-ed composited polynomial on two variables? No, I won't.  
So, as the math is right, I have:

$$X = \left(A\circ X\right)^{-1}\times P$$

Thus, assuming I can do it (maybe I can't... who knows?):

$$X_0 = P$$

$$X_{n + 1} = \left(A\circ X_n\right)^{-1}\times P$$

With:

- $X$
  - Point at the input / original image
- $P$
  - The point at the new / destination image

| Who? | Approach |
| ---- | -------- |
| Me (original) | $P = A\circ(X) \times X$ |
| OpenCV | $X = A^{-1}\times P$ |
| Me (new) | $X = \left(A\circ X\right)^{-1}\times P$ |

My code stopped at implementing this step.

### Post remapping

Then the image is remapped back:

$$\left[0; 1\right[_\mathbb{R}^2\to\left(\left[0; \mathrm{new\, width}\right[_\mathbb{N}\times\left[0; \mathrm{new\, height}\right[_\mathbb{N}\right)$$

And finally, saved.

### Bonus: background work

I could implement something like that...
