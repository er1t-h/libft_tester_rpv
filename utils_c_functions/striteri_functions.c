#include <ctype.h>

void	rotx_in_place(unsigned int i, char *s) {
	if (isupper(*s)) {
		*s = ((long)(*s - 'A') + i) % 26 + 'A';
	} else if (islower(*s)) {
		*s = ((long)(*s - 'a') + i) % 26 + 'a';
	}
}

void	rotone_in_place(unsigned int i, char *s) {
	(void) i;
	if (isupper(*s)) {
		*s = ((long)(*s - 'A') + 1) % 26 + 'A';
	} else if (islower(*s)) {
		*s = ((long)(*s - 'a') + 1) % 26 + 'a';
	}
}

void	to_num_in_place(unsigned int i, char *s) {
	if (isupper(*s)) {
		*s = ((long)(*s - 'A') + i) % 10 + '0';
	} else if (islower(*s)) {
		*s = ((long)(*s - 'a') + i) % 10 + '0';
	}
}
