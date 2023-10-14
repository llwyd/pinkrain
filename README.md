# pinkrain

Real time pink noise generation with rain visualization.

![pinkrain](assets/pinkrain.gif)

# Algorithm
The _Voss-McCartney_ algorithm is used to generate the pink noise.  The algorithm essentially involves having multiple white noise generators that each update at different rates, as detailed in the gif below. I've found that 15 generators (plus a continually updateing white noise generator) produces a pleasant sounding pink noise.


Included in this repository is a model of the algorithm, which shows that if you average several runs of the algorithm you get a pretty close to ideal pink noise frequency response.  There are some noticible ripples there but it's insignificant to the human ear :).

# Resources
* [1] [DSP generation of Pink (1/f) Noise](https://www.firstpr.com.au/dsp/pink-noise)
* [2] [Generating pink noise](https://www.dsprelated.com/showarticle/908.php)
