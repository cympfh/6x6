#!/bin/bash

SIX=./target/release/6x6

[ -d /tmp/6x6 ] || mkdir /tmp/6x6
BD=/tmp/6x6/self.bd
RESULT=/tmp/6x6/self.result

init() {
    cat <<EOM >$BD
......
......
..ox..
..xo..
......
......
EOM
}

solve() {
    echo "CPU> I'm thinking..."
    ( echo "$1"; cat $BD) | $SIX solve >$RESULT
    if [ "$(cat "$RESULT")" = "pass" ]; then
        echo "CPU> I have no choice. Pass."
    else
        HAND=$(tail -n 7 $RESULT | head -1)
        echo "CPU> $HAND"
        tail -n 6 $RESULT >$BD
    fi
}

display() {
    echo "  a b c d e f"
    < $BD sed 's/./ &/g' | awk '{print NR $0}'
}

check() {
    $SIX check <$BD >$RESULT
    if [ "$(head -1 "$RESULT")" = "end" ]; then
        echo "$(tail -1 "$RESULT") won!!"
        true
    else
        false
    fi
}

# main

init
NEXT=o
PREV=x

while :; do

    solve "$NEXT"
    display
    if check; then
        break
    fi

    # swap
    TMP=$NEXT
    NEXT=$PREV
    PREV=$TMP
done
