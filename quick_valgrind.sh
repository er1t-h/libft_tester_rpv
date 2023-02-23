executable=`ls -t ./target/debug/deps/ | grep -E "^libft_rpv-[0123456789abcdef]+$" | head -n 1`
valgrind --leak-check=full --show-leak-kinds=all --suppressions=./valgrind_suppression_files/patch_rusty_and_cargo_test.txt target/debug/deps/${executable} $1
