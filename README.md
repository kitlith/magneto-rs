# magneto-rs
Pure rust reimplementation of magneto, an elipsoid fitter used for calibrating magnetometers and accelerometers.

The implementation is based off of the [copy used in the SlimeVR Firmware](https://github.com/SlimeVR/SlimeVR-Tracker-ESP/blob/9d93df6e6a0f3c0f68466669a100667925b4d801/lib/magneto/magneto1.4.cpp).

The main difference from typical prior implementations is the ability to feed an infinite number of samples to the algorithm without taking up any additional memory. On the flip side, no effort has been made yet to optimize stack usage while finalizing the resulting matrix, unlike the implementation I referenced that meticulously allocated and freed matricies.

In theory, we could reduce the memory requirement even further [using some additional tricks](https://blog.demofox.org/2017/01/02/incremental-least-squares-surface-and-hyper-volume-fitting/) from the post that inspired this approach. In the end, I settled with a simpler means of getting what I wanted, since this is functional and still only requires constant memory usage. The post implies that the matrix should be a hankel matrix, but it seems to only be a symmetric matrix. *shrug*