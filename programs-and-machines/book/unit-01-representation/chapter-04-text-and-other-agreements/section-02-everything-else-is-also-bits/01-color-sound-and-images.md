# Color, Sound, and Images

Text was hard because writing is a human institution with thousands of years of
accumulated irregularity. Color and sound are easier in one way — they are
physical quantities — and harder in another, because they are *continuous*, and
we already know what a finite machine does to continuous things.

## Color

Light arriving at your eye is a continuous distribution of energy across
wavelengths. A full description would be a curve, and a curve needs infinitely
many numbers.

The trick is that human eyes do not measure the curve. The retina has three types
of cone cell, sensitive to roughly long, medium, and short wavelengths, and
whatever the incoming light is, the brain receives three numbers. Two physically
different spectra that produce the same three responses look identical — a pair
called **metamers**.

So we do not need the curve. We need three numbers, and the standard choice is
the response to red, green, and blue.

The usual encoding gives each channel one byte:

```
red    0–255
green  0–255
blue   0–255
```

Three bytes per pixel, and $256^{3}$ = 16,777,216 distinguishable colors — the "16.7
million colors" of advertising copy. Written in hexadecimal, as Chapter 2
promised, `#FF8800` is red 255, green 136, blue 0.

Now the familiar consequences. There are more distinguishable colors in the world
than 16.7 million, so the encoding quantizes: nearby colors collapse onto the same
triple. This is usually invisible, but on a smooth gradient — a clear sky, a soft
shadow — you can sometimes see it as **banding**, where a continuous ramp becomes
visible stripes. It is the same phenomenon as floating-point rounding, in a
different medium.

And the choice of three numbers is a choice. It encodes an assumption about the
observer. It works because human eyes have three cone types; it would be wrong for
a mantis shrimp, and it is wrong for scientific instruments, which is why
astronomy and remote sensing use hyperspectral images with dozens of channels
rather than three.

## Sound

Sound is a pressure wave: air pressure varying continuously with time. Again, a
curve; again, infinitely many numbers.

The move here is **sampling** — measure the pressure at regular intervals and
record the measurements. Two decisions follow.

**How often to measure?** This one has a precise answer. The Nyquist–Shannon
sampling theorem says that to reconstruct a signal containing frequencies up to
*f*, you must sample at a rate greater than 2*f*. Human hearing reaches about
20 kHz, so a rate above 40 kHz suffices. Compact discs use 44,100 samples per
second, the excess being room for the filters that enforce the limit.

This is a genuinely remarkable result. It says that a continuous wave, sampled at
discrete instants, can be reconstructed *exactly* — not approximately — provided
it contains no frequencies above half the sampling rate. Some information is not
lost by sampling. Which information depends entirely on the rate you chose.

Sample too slowly and you get **aliasing**: high frequencies masquerade as low
ones, because the samples are consistent with both. It is the same effect that
makes wagon wheels appear to spin backwards in films, where the sampling rate is
the frame rate.

**How precisely to measure each sample?** CDs use 16 bits, giving 65,536 levels.
The error introduced by rounding each sample to the nearest level is quantization
noise, and 16 bits puts it below the threshold of hearing for most material.
Studio work often uses 24 bits, for the same reason you keep extra digits during
a long calculation: repeated processing accumulates error, and headroom protects
against it.

Multiply out: 44,100 samples per second × 16 bits × 2 channels = 1,411,200 bits
per second. A minute of CD audio is about 10.1 MiB, which is why compression was
not optional in 1999 and why MP3 changed the music industry.

## Images

An image is a continuous field of color over a two-dimensional area, and the
approach is the same one: sample it on a grid. Each sample is a **pixel**, and the
same two questions recur.

**How fine a grid?** Too coarse and you see the samples — visible blockiness, and
in photographs of regular patterns a spatial aliasing effect called a moiré
pattern, which is exactly the wagon-wheel problem in two dimensions.

**How precisely per sample?** Usually the three bytes from the color discussion,
sometimes a fourth for transparency.

A 1920 × 1080 image at three bytes per pixel is 6,220,800 bytes — about 5.9 MiB.
Thirty frames a second of that is roughly 1.5 gigabits per second, which no
consumer network has ever carried. Video is therefore always compressed, and the
compression is lossy, which means every video you have ever watched was an
approximation whose errors were engineered to fall where your eye does not look.

## The pattern

Three media, one procedure.

1. Choose what to measure. Three color channels; air pressure; a grid of points.
   This step encodes an assumption about the observer, and it is the step people
   forget is a choice.
2. Choose how finely to measure — the resolution, the sample rate, the bit depth.
3. Accept that everything between the measurements is gone, and understand the
   characteristic way in which it is gone: banding, aliasing, quantization noise.

You have now seen this five times. Integers quantize the number line and wrap at
the edges. Floating point quantizes with a spacing that grows with magnitude.
Text quantizes the infinite variety of written language into a code-point table.
Color quantizes a spectrum into three bytes. Sound quantizes a wave into samples.

Different subjects, same move. The next lesson says what the move is.
