#!/bin/bash

SIX=$HOME/git/6x6/target/release/6x6

TMP_USERHAND=/tmp/six.userhand
TMP_AIHAND=/tmp/six.aihand
TMP_BOARD=/tmp/six.board
TMP=$(mktemp)

usage() {
    cat <<EOM
six new -- new game
six status -- show the current
six <position> -- put your stone (position is like "b3")
six pass -- pass
EOM
    exit
}

display() {
    echo "_ abcdef"
    paste <(echo "123456" | grep -o .) $TMP_BOARD | sed 's/\t/ /g' | sed 's/\./+/g'
}

solve() {
    (
        cat $TMP_AIHAND
        cat $TMP_BOARD
    ) | $SIX solve 2>/dev/null >$TMP
    if [ $( head -1 $TMP ) == pass ]; then
        echo pass
    else
        cat $TMP | tail -n 6 > $TMP_BOARD
    fi
}

new() {

    cat <<EOM > $TMP_BOARD
......
......
..ox..
..xo..
......
......
EOM

    if [ $(( RANDOM % 2 )) -eq 0 ]; then

        echo "You are x"
        echo x > $TMP_USERHAND
        echo o > $TMP_AIHAND
        solve
        display

    else

        echo "You are o"
        echo o > $TMP_USERHAND
        echo x > $TMP_AIHAND
        display

    fi
    echo "put <position (e.g. c2)> or pass"
}

check() {
    $SIX check <$TMP_BOARD >$TMP
    if [ $( head -1 $TMP ) == end ]; then
        echo "End: $( tail -1 $TMP ) won"
        display
        exit
    fi
}

put() {

    # validation
    if [ $(echo "$1" | sed 's/^[a-f]/@/; s/[1-6]$/@/g') == "@@" ]; then
        :
    else
        cat <<EOM
The position ($1) is invalid.
EOM
        usage
        exit 1
    fi


    check

    (
        cat $TMP_USERHAND
        echo $1
        cat $TMP_BOARD
    ) | $SIX put >$TMP

    if [ $(head -1 $TMP) == ok ]; then
        tail -n 6 $TMP > $TMP_BOARD
        check
        solve
        check
    else
        echo "invalid"
    fi

    display
}

pass() {
    check
    solve
    check
    display
}


if [ $# -eq 0 ]; then
    usage
fi

case "$1" in
    new | init )
        new
        ;;
    pass )
        pass
        ;;
    status )
        echo "You are $(cat $TMP_USERHAND)"
        check
        display
        ;;
    help | --help | -h )
        usage
        ;;
    * )
        put "$1"
        ;;
esac
