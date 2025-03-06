
A sudoku to Conjunctive normal form translator.

Takes in a sudoku on stdin in the format:
```
.49.6....
.....8.63
..2..5...
...9....8
8..2.7.9.
..6.8...4
9..8..1..
57....8..
......5..
```
Note that the `.`'s just need to be any different character than the numbers 1 to 9.
Satdoku then prints the contents for the cnf file to stdout.
This means that if you have a file containing a sudoku like above you can
translate it using `cat sudoku.txt | satdoku > sudoku.cnf`.
If your sat solver supports input over stdin you can even do `cat sudoku.txt | satdoku | solver`
to get the result.
