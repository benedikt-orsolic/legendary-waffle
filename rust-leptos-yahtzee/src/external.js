export function rng() {
	const tmp = Math.random() * 10 % 6 + 1;
	console.log(tmp);

	return tmp;
}
