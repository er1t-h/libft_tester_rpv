rg -A 5 "alloc\(" . ../ -g '*.c'
echo "You should make sure that each malloc/calloc is protected, and that every memory is freed"
echo "Some people might use functions that do not end by `alloc` to allocate, trying to dodge this kind of test. Please make sure there is none in this project."
