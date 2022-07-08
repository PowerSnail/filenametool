#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

bin="${SCRIPT_DIR}/../target/debug/filenametool"

# Set up test files

root=$(mktemp -d -t "filenametool-tests.XXXXXXXXXX")
cd "$root" || exit
root_abs=$(realpath "$root")

mkdir -p a/b/c/d
touch a/exists.txt
touch a/b/exists.txt
touch a/b/c/exists.txt
touch a/b/c/d/exists.txt

ln -s a/exists.txt a/exists.link

heading() {
    command="$1"
}

error="Failure"
count=0
count_failure=0

test() {
    local length
    local result

    count=$((count + 1))
    length=$(($# - 1))
    result=$("$bin" "$command" "${@:1:$length}" || echo $error)

    if [ "$result" == "${*: -1}" ]; then
        echo "$command"
    else
        count_failure=$((count_failure + 1))
        {
            printf "%10s %s\n" "Command:" "$command"
            printf "%10s [" "Args:"
            for a in "${@:1:$length}"; do
                printf "\"%s\", " "$a"
            done
            printf "]\n"
            printf "%10s \"%s\"\n" "Got:" "$result"
            printf "%10s \"%s\"\n" "Expected:" "${*: -1}"
            printf "\n"
        } >>failure.txt
    fi
}

heading canonicalize

test "a" "$root_abs/a"
test "./a" "$root_abs/a"
test "a/b" "$root_abs/a/b"
test "a/exists.txt" "$root_abs/a/exists.txt"

heading component

test "a" "0" "a"
test "a/b" "1" "b"
test "a/b/c/d" "0" "a"
test "a/b/c/d" "1" "b"
test "a/b/c/d" "2" "c"
test "a/b/c/d" "3" "d"
test "/" "0" "/"
test "./" "0" "."

heading exists

test "a/" ""
test "a/exists.txt" ""
test "a/b" ""
test "a/b/exists.txt" ""
test "a/b/c" ""
test "a/b/c/exists.txt" ""
test "a/b/c/d" ""
test "a/b/c/d/exists.txt" ""

test "not_exist/" "$error"
test "a/not_exists.txt" "$error"
test "a/b/not_exists.txt" "$error"
test "a/b/c/not_exists.txt" "$error"
test "a/b/c/d/not_exists.txt" "$error"

heading extension

test "a/" "$error"
test "a/a.txt" "txt"
test "a/a.txt.tar" "tar"

heading filename

test "a/" "a"
test "/" "$error"
test "a/a.txt" "a.txt"

heading is-absolute

test "/" ""
test "/a/a.txt" ""
test "a/" "$error"
test "a/a.txt" "$error"

heading is-dir

test "/" ""
test "a/" ""
test "a" ""
test "a/exists.txt" "$error"
test "not_exist" "$error"
test "/a/not_exist.txt" "$error"

heading is-file

test "a/exists.txt" ""
test "a/b/exists.txt" ""
test "a/b/c/exists.txt" ""
test "/" "$error"
test "a/" "$error"
test "a" "$error"
test "not_exist" "$error"
test "/a/not_exist.txt" "$error"

heading is-relative

test "/" "$error"
test "/a/a.txt" "$error"
test "a/" ""
test "a/a.txt" ""

heading is-symlink

test "a/exists.link" ""
test "/" "$error"
test "/a/a.txt" "$error"
test "a/" "$error"

heading join

test "a" "b" "c" "a/b/c"
test "a" "exists.txt" "a/exists.txt"
test "a" ".." "exists.txt" "a/../exists.txt"
test "." ".." "exists.txt" "./../exists.txt"
test "a" "b" "/exists.txt" "/exists.txt"
test "a/" "b/" "exists.txt" "a/b/exists.txt"

heading parent

test "a/exists.link" "a"
test "a/b/exists.link" "a/b"
test "a/b/c/exists.link" "a/b/c"
test "./" ""
test "/" "$error"
test "" "$error"

heading resolve-link

test "a/exists.link" "a/exists.txt"
test "a/exists.txt" "$error"

heading stem

test "a/exists.link" "exists"
test "/" "$error"
test "./" "$error"
test "" "$error"
test "a" "a"

heading with-file-name

test "a/exists.link" "x" "a/x"
test "/" "x" "/x"
test "./" "x" "./x"
test "" "x" "x"
test "a" "x" "x"

heading with-suffix

test "a/exists.link" "txt" "a/exists.txt"
test "a/" "txt" "a.txt"
test "/" "txt" "$error"
test "./" "txt" "$error"
test ""  "txt" "$error"

if [ $count_failure == 0 ]; then
    printf "All passed.\n"
else
    printf "%d errors found.\n\n" $count_failure
    cat failure.txt
fi
