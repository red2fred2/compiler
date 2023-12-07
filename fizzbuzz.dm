remainder: (x: int, y: int) int {
	return x - y * (x / y);
}

fizz_buzz: (n: int) void {
	i: int = 1;

	while(i <= n) {
		is_word: bool = false;

		// Fizz
		if(remainder(i, 3) == 0) {
			is_word = true;
			give "Fizz";
		}

		// Buzz
		if(remainder(i, 5) == 0) {
			is_word = true;
			give "Buzz";
		}

		// Otherwise, print i
		if(!is_word) {
			give i;
		}

		give "\n";
		i++;
	}
}

main: () void {
	n: int;
	take n;

	fizz_buzz(n);
}
