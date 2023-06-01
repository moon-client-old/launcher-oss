/* used for ids */
export const jsf32 = (a: number, b: number, c: number, d: number) => (min: number = 0, max: number = 1) => {
	a |= 0; b |= 0; c |= 0; d |= 0;
	const t = a - (b << 27 | b >>> 5) | 0;
	a = b ^ (c << 17 | c >>> 15);
	b = c + d | 0;
	c = d + t | 0;
	d = a + t | 0;
	if (min > max) {
		const storage = min;
		min = max;
		max = storage;
	}
	return ((d >>> 0) / 4294967296) * (max - min) + min;
};
export default jsf32;