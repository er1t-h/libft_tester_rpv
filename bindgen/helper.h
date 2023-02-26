#pragma once

void	rotx_in_place(unsigned int i, char *s);
void	rotone_in_place(unsigned int i, char *s);
void	to_num_in_place(unsigned int i, char *s);

char	rotx(unsigned int i, char c);
char	rotone(unsigned int i, char c);
char	to_num(unsigned int i, char c);

void	nofree(void *data);
void	all_plus_one(void *elt);
void	*times_two(void *elt);
