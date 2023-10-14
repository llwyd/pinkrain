import numpy as np
import matplotlib.pyplot as plt
from scipy import signal
from scipy.io import wavfile
from scipy import stats
import random
from tqdm import trange

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

def get_slope( x, y ):
    slope, _, _, _, _ = stats.linregress( x, y )
    return slope

def norm( n ):
    return n/np.max(np.abs(n))

def fft(x,fs,fft_len,norm=None):
    F = np.fft.fft(x,fft_len,norm=norm)
    F = np.abs(F)
    Ff = (fs/2)*np.linspace(0,1,int(fft_len/2))
    Fdb = 20*np.log10(F[:int(len(F)/2)]);
    
    return F, Ff, Fdb

def generate_decade_line(start_mag, end_freq):
    iterations = int(np.log10( end_freq ) )
    iterations += 1

    mags = np.zeros(iterations)
    freqs = np.zeros(iterations)

    for i in range( iterations ):
        mags[i] = start_mag
        freqs[i] = ( 10 ** i )

        start_mag -= 10

    return mags, freqs

def gain( g ):
    return np.power(10, g / 20 )

def get_fslope( Xf, Xdb ):
    slope, _, _, _, _ = stats.linregress( np.log10( Xf, where=Xf > 0 ), np.log10( gain( Xdb ) ) )
    return slope

def voss(num_samples,generators):
    assert ( (generators+1) & ((generators+1) - 1) ) == 0
    shift = np.uint32(np.log2(generators+1))
    rollover = 2**( generators-1 )
    assert trailing_bits(rollover) == (generators-1)
    noise_array = []
    
    x = np.zeros(num_samples)
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

        counter = ( counter & (rollover - 1) )
        counter = counter + 1

    return x, indices

fs = 44100
num_samples = fs
num_tests = 1000
generators = 15

print(f'Voss-McCartney Pink Noise Generator')
print(f'    Sample Rate: {fs}') 
print(f'         Length: {num_samples} Samples ({num_samples*(1/fs):.2f}s)') 
print(f'  Noise sources: {generators}')
print(f'Test iterations: {num_tests}')
print(f'Generating...')

Ydb = np.zeros(int(num_samples/2))
Ydb_32 = np.zeros(int(num_samples/2))
Yf = []

for i in trange(num_tests):
    x, indices = voss(num_samples,generators)
    x = norm(x)
    X, Xf, Xdb = fft(x, fs, len(x),norm='ortho' )
    Ydb = np.add(Ydb,Xdb)

Zdb = Ydb / num_tests

ideal_db, ideal_f = generate_decade_line(20, 100000)
X_slope = get_fslope( Xf, Xdb );
ideal_slope = get_fslope( ideal_f, ideal_db);

indices_used, instances = np.unique(indices, return_counts=True)
print(f'\nIndex Analysis')
for i in range(len(indices_used)):
    print(f'{indices_used[i]} = {instances[i]} ({instances[i]/num_samples:.5f}%)')
print(f'\nGradient Analysis')
print(f' Pink Slope: {X_slope:.2f}')
print(f'Ideal Slope: {ideal_slope:.2f}')


plt.figure(1)
plt.semilogx( Xf, Xdb )
plt.semilogx( Xf, Zdb )
plt.semilogx( ideal_f, ideal_db )
plt.legend(['Single iteration', 'Average of all iterations', 'Ideal 1/f'])
plt.xlim( 1, int(fs / 2 ) )

plt.grid(which='both')

plt.title('Pink Noise Model (Voss-McCartney)')
plt.ylabel('Magnitude (dB)')
plt.xlabel('Frequency (Hz)')

plt.show()

