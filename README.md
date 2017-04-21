# 6x6 reversi

`6x6` is a basic tool for 6x6 reversi.

## solver

`solver` searchs the best choice for the game.

### Input

An input consits of 3 parts.
The 1st line must be `solve` keyword (this specify solver mode).
The 2nd line is the next player of the game.

The latter after 3rd line is the 6x6 board,
which is 6 lines and a line is 6 chars.
The char is one of `.`, `o` or `x`.
`.` is empty cell, 'o' is the left player cell
and 'x' is the right player cell.

__N.B.__
`o` is assumed to be the first player of the game.

```
solve
o
......
......
..ox..
..xo..
......
......
```

which is the next player to the input

### Output

The output is the best choice of the next player,
and the game after the choice.

```
b4
......
......
..ox..
.ooo..
......
......
```

#### N.B.

```
a1 b1 c1 d1 e1 f1
a2 b2 c2 d2 e2 f2
a3 b3 .. .. ..
a4 b4 .. ..
a5 .. ..
a6 b6 c6 d6 e6 f6
```

## put

`put` just puts a stone.

### Input

```
put
o
b4
......
......
..ox..
..xo..
......
......
```

### Output

```
ok
......
......
..ox..
.ooo..
......
......
```

### Input

```
put
o
a4
......
......
..ox..
..xo..
......
......
```

### Output

```
invalid
```

