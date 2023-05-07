My name is Yann Youbi. I work individually

I have received help from the professor in class, to understand how bitpacking works as well as compression and decompression in general.
I have received help from my TA, who has provided me with rpeg files to decompress in order to test my program

I honestly like the way my program is organized and implemented, from my tests everything seems to work. That doesn't mean it's perfect as I could be amazed by another implementation but the only implementation I have seen so far is mine.

My implementation is composed of 6 major components inside of my rpeg:
	cv_rgb: does the conversions from rgb to component video and from component video to rgb
	un_block: does 2 by 2 block iteration on an array 2 and returns an array 2 of vector with each vector representing a block. Can also turn an array 2 of block vectors back to a regular array of elements with no block
	av_dct: uses un_block to turn (y, pb,pr) into ((a, b, c, d), pb average, pr average) and turns ((a, b, c, d), pb average, pr average) into (y, pb, pr). Quantize or unquantize pb, pr averages in the process from float to usize
	un_pack: turns (a, b, c, d) from float to unsigned integer u64 for a and signed integer i64 for b, c and d. Does the opposite as well
	output: uses bitpack function to turns ((a, b, c, d), pb, pr) into a u32 word
	codec: has 3 fonctions: compress, decompress, compress_and_decompress that I've added. They call the previours function in order in order to compress a files, decompresse a file or do both at the same time from a file name. prints it to standard output

Bitpack folder provided that I've implemented in order to store a value into a word or get it from a word for signed and unsigned integers.
	I've added 2 functions: 
	  - new_all: calling news/newu on each element of my tuple in order to turn ((a, b, c, d), pb, pr) into a word u32
	  - get_all: calling gets/getu a u32 word mutiple times the required width and lbs in order to get a tuple ((a, b, c, d), pb, pr) out of it.

I haven't really timed myself. I believe it hasn't taken me more than a couple hours to analyze the problem posed by the assignment,  especially thanks to the explanations of the professor.
However it did take me quite some time (more than a couple hours) to understand the role of each function present in bitpack and how to I would implement it.

It did take me about 3 days solving the problem. 1 day for my first version of the bitpack module, 1 hour for my second version and 2 days for everything else. The un_block module to expand a blocked array 2 of vectors back to its original form unblocked took me quite some time especially finding the right functions from standard library to do it.

(I have 2 different implementations of that bitpack module, the more recents one that I've submitted takes less lines and it just took me an hour to implement)