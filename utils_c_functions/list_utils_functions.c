#include <ctype.h>
#include <string.h>

void	nofree(void *data) {
	(void) data;
}

void	all_plus_one(void *elt) {
	char *value = (char *) elt;
	while (*value) {
		if (isdigit(*value))
			*value = (*value - '0' + 1) % 10 + '0';
		value++;
	}
}

void	*times_two(void *elt) {
	unsigned long long value = (unsigned long long) elt;
	return (void *) (value * 2);
}
