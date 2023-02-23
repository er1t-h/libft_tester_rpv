#include <ctype.h>

char	rotx(unsigned int i, char c) {
	if (isupper(c)) {
		return ((long)(c - 'A') + i) % 26 + 'A';
	} else if (islower(c)) {
		return ((long)(c - 'a') + i) % 26 + 'a';
	}
	return c;
}

char	rotone(unsigned int i, char c) {
	(void) i;
	if (isupper(c)) {
		return ((long)(c - 'A') + 1) % 26 + 'A';
	} else if (islower(c)) {
		return ((long)(c - 'a') + 1) % 26 + 'a';
	}
	return c;
}

char	to_num(unsigned int i, char c) {
	if (isupper(c)) {
		return ((long)(c - 'A') + i) % 10 + '0';
	} else if (islower(c)) {
		return ((long)(c - 'a') + i) % 10 + '0';
	}
	return c;
}
