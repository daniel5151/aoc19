set -e
RUSTFLAGS=

DAY=$1
QUESTION=$2
INPUTFILE=./inputs/$DAY.txt

if [ ! -f $INPUTFILE ]; then
    curl "https://adventofcode.com/2019/day/$DAY/input" -H "cookie: $(cat cookie.txt)" --compressed > $INPUTFILE
fi

cargo run -- $@
