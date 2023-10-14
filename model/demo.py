import numpy as np
import matplotlib.pyplot as plt
from scipy import signal
from scipy.io import wavfile
from scipy import stats
import random
from tqdm import trange
import matplotlib.animation as animation

class NoiseGenerator():
    def Update(self):
        self.prev_value = self.value
        self.value = (random.random() * 2.0) - 1.0

    def __init__(self):
        self.value = 0.0
        self.prev_value = 0.0

def trailing_bits(num):
    bits = bin(num)
    return len(bits) - len(bits.rstrip('0'))

def voss(num_samples,generators):
    assert ( (generators+1) & ((generators+1) - 1) ) == 0
    shift = np.uint32(np.log2(generators+1))
    rollover = 2**( generators-1 )
    assert trailing_bits(rollover) == (generators-1)
    noise_array = []
    
    x = np.zeros(num_samples)
    y = np.zeros([generators + 1, num_samples]);
    white = 0.0
    white_noise = NoiseGenerator()
    white_noise.Update()

    for i in range(generators):
        noise_array.append(NoiseGenerator())
        noise_array[i].Update()
        white = white+noise_array[i].value

    white = white+white_noise.value

    # Voss-McCartney pink noise algorithm
    counter = 1
    indices = np.zeros(num_samples)
    for i in range(num_samples):
        index = trailing_bits(counter)
        indices[i] = index
   


        noise_array[index].Update()
        white_noise.Update()

        white = white - white_noise.prev_value
        white = white + white_noise.value

        white = white - noise_array[index].prev_value;
        white = white + noise_array[index].value;
        x[i] = white

        x[i] = x[i] / (generators+1)
        
        y[0][i] = white_noise.value;
        for j in range(generators):
            y[j + 1][i] = noise_array[j].value;

        counter = ( counter & (rollover - 1) )
        counter = counter + 1

    return x, y

fs = 44100
num_samples = 1024
generators = 15

x, y = voss(num_samples,generators)

t = np.linspace(0,num_samples - 1,num_samples)
max_frames = 256

fig, ax = plt.subplots(nrows=5,ncols=1)

fig.suptitle("Voss-McCartney noise generators", fontsize=18)
line = ax[0].plot(t[0],y[0][0])[0]
ax[0].set(xlim=[0,max_frames - 1],ylim=[-1,1])
ax[0].set_yticks([])
ax[0].set_xticks([])

line1 = ax[1].plot(t[0],y[1][0])[0]
ax[1].set(xlim=[0,max_frames - 1],ylim=[-1,1])
ax[1].set_yticks([])
ax[1].set_xticks([])

line2 = ax[2].plot(t[0],y[2][0])[0]
ax[2].set(xlim=[0,max_frames - 1],ylim=[-1,1])
ax[2].set_yticks([])
ax[2].set_xticks([])

line3 = ax[3].plot(t[0],y[3][0])[0]
ax[3].set(xlim=[0,max_frames - 1],ylim=[-1,1])
ax[3].set_yticks([])
ax[3].set_xticks([])

line4 = ax[4].plot(t[0],y[4][0])[0]
ax[4].set(xlim=[0,max_frames - 1],ylim=[-1,1])
ax[4].set_yticks([])
ax[4].set_xticks([])

def update(frame):
    line.set_xdata(t[:frame])
    line1.set_xdata(t[:frame])
    line2.set_xdata(t[:frame])
    line3.set_xdata(t[:frame])
    line4.set_xdata(t[:frame])
    
    line.set_ydata(y[0][:frame])
    line1.set_ydata(y[1][:frame])
    line2.set_ydata(y[2][:frame])
    line3.set_ydata(y[3][:frame])
    line4.set_ydata(y[4][:frame])


ani = animation.FuncAnimation(fig = fig, func=update, frames = max_frames, interval = 20)

ani.save(filename="../assets/noise_gen.gif", writer="pillow")
#plt.show()

