#!/bin/bash

SIX=./target/release/6x6

[ -d /tmp/6x6 ] || mkdir /tmp/6x6
BD=/tmp/6x6/bd
RESULT=/tmp/6x6/result

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
    ( echo "$ME"; cat $BD) | $SIX solve >$RESULT
    HAND=$(tail -n 7 $RESULT | head -1)
    echo "CPU> $HAND"
    tail -n 6 $RESULT >$BD
}

display() {
    echo "  a b c d e f"
    < $BD sed 's/./ &/g' | awk '{print NR $0}'
}

put() {
    ( echo "$YOU"; echo "$1"; cat $BD) | $SIX put >$RESULT
    if [ "$(head -1 "$RESULT")" = "invalid" ]; then
        false
    else
        tail -n 6 $RESULT >$BD
    fi
}

check() {
    $SIX check <$BD >$RESULT
    if [ "$(head -1 "$RESULT")" = "end" ]; then
        if [ "$(tail -1 "$RESULT")" = "$YOU" ]; then
            echo "You won!"
        else
            echo "I won!!!"
        fi
        true
    else
        false
    fi
}

# main

init

if [ $((RANDOM % 200)) -eq 0 ]; then
    YOU=o
    ME=x
    echo "== You are o. It's your turn."
else
    YOU=x
    ME=o
    echo "== You are x. First, it's CPU turn."
    solve
fi

while :; do

    display

    if check; then
        break
    fi

    while :; do
        echo -n "YOU> "
        read HAND
        if [ "$HAND" = "pass" ]; then
            break
        fi
        put "$HAND"

        if [ $? -eq 0 ]; then
            break
        else
            echo invalid
        fi
    done

    display

    if check; then
        break
    fi

    solve

done
