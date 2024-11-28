# A `libft` tester in Rust

This is a tester for the `libft` project of school 42.
It benefits from the speed of Rust's testing framework, allowing to launch all tests in under a second.

## Warning

Of course, you shouldn't use that as a student. I know I cannot prevent you to use it. But I mean it. There are maybe cases this tester doesn't handle. And you should always write your own tests.

## Setup

1. Download the release of this tester inside the student's libft project.
2. Add this in the Makefile of the student (of course, change the `${OBJS}` if needed):
```Makefile
.PHONY: dynamic_lib
dynamic_lib:
    cc -shared -o libft.so ${OBJS}
```
3. Run `make dynamic_lib`. If it fails, just run the command yourself with all of the student `.o` files.
4. Run `./libft_tester`. If there's a lot of red, it might be because the person you correct didn't implement the bonus, in which case you can run `./libft_tester mandatory`.
    - If *every single test fails* (with a lot of mention of "poisoning"), it might be because it didn't find the `libft.so` file. Ensure the tester is placed in the same directory as the `.so`.
5. To run valgrind (because you definitely want to run valgrind), use this command (don't forget to download the suppression file from the repo!):
```sh
valgrind --leak-check=full --trace-children=yes --show-leak-kinds=all --log-file=libft_leaks.log --suppressions=./valgrind_suppression_files/patch_rusty_and_cargo_test.txt ./libft_tester mandatory
```
	- Of course, check the `libft_leaks.log` file after running `valgrind`
6. **If a test fails, and the error message contains "DPS:", it's not the student fault**. Therefore, Don't Penalize the Student.
7. If a test fails, and you cannot locate the issue at all, please open an issue. I might have let something slip.
8. Is the student's libft holding up? If yes, run `LIBFT_TESTER_RANDOM_REPEAT=5000 ./libft_tester`. You see, some of the tests are randomly generated. By default, each random test is run 10 times. But by modifying the `LIBFT_TESTER_RANDOM_REPEAT` environment variable, you can run *way more* tests. Have fun! (however valgrind *really* slows down the execution time, so maybe don't do 5000 tests with valgrind... on my computer it takes ~5 minutes)

## Setup (copy-paste and execute)

This will run all 134 tests, with 5000 different inputs for each of the 48 random tests, then relaunch the tests with valgrind and 10 inputs/random test.
First, add the `-fPIC` flag to the compile flags (next to `-Wall -Wextra -Werror`). Run `make fclean` and `make bonus`. Then copy-paste:

```sh
cc -o libft.so -shared -Wl,--whole-archive libft.a -Wl,--no-whole-archive
wget https://github.com/er1t-h/libft_monkey_tester/releases/latest/download/libft_tester
chmod +x libft_tester
LIBFT_TESTER_RANDOM_REPEAT=5000 ./libft_tester
if [ "$?" -eq 0 ]
then
	wget https://raw.githubusercontent.com/er1t-h/libft_monkey_tester/refs/heads/main/valgrind_suppression_files/patch_rusty_and_cargo_test.txt
	valgrind --leak-check=full --show-leak-kinds=all --suppressions=patch_rusty_and_cargo_test.txt --log-file=libft_monkey_tester.log ./libft_tester
	if [ "$?" -eq 0 ]
	then
		cat libft_monkey_tester.log
	else
		echo "You failed the valgrind tests"
	fi
else
	echo "You failed some tests"
fi 
```
