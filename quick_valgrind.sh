executable=`ls -t ./target/debug/deps/ | grep -E "\$libft_rpv-" | head -n 1`
valgrind --leak-check=full --show-leak-kinds=all --suppressions=./valgrind_suppression_files/cargo_default_leak.txt --suppressions=./valgrind_suppression_files/rusty_fork.txt --gen-suppressions=all target/debug/deps/${executable}
