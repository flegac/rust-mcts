# Rust Monte-Carlo Tree Search algorithm

# Features

- MCTS API + implementation
- Go game board structure
- Go game engine

# About

The project is composed of the followings crates :

- go-game: playing go with Monte-Carlo Tree Search
- go-lib: Go game library & data structures

- graph-lib: tree & graph structures
- mcts-lib: MCTS API & implementation

- rust-tools: logging & benchmark helpers

# Usage

Build & run go-game to look at some auto-generated games.

Hum... Do not expect wonders !

# TODO

***** CONTINUE graph-lib/src/tree2/mod.rs !! (the new MCTS) *****

- State copy
- multi-threading : simulations

- CORRECT RULES ! (check position against other application - using SGF output)
- Ko rules
- suicide move : adding a stone that kill itself is possible.

- improve SGF support
- LOAD SGF !!

- Optimise Layout speed (String creation & copies)
- Implements GoEditor : command line interface to edit & visualise GoState/GoBoard

# Issues

- Correct end game (pass/pass)

# Exemples

```
[X] A B C D E F G H I J K L M [X]
  + --------------------------+  
a | X X . X X X . X X X X O O | a
b | O X X O O X X O X O O . O | b
c | O X X O X X . O O O O O X | c
d | O O . O X X . X O O O . X | d
e | O X X . O X O O X O O O . | e
f | O X X O O X O X X X O X X | f
g | . O X O O X X X X X O X O | g
h | O O X O O X X O O X X O . | h
i | O O O O O O X O X X X X X | i
j | O O O O O . O O O X . X . | j
k | O O . X X O O O . O X O X | k
l | O O X . X X X O X X O O . | l
m | X X X X X X X X O O O O X | m
  + --------------------------+  
[X] A B C D E F G H I J K L M [X]
black: territories=0, captured=11
white: territories=0, captured=8
X: 74 stones, 12 groups
O: 77 stones, 11 groups
.: 18 stones, 17 groups
history(170):
;Lj;Ll;Bj;Lm;Gl;Cj;Ff;Jk;Gh;Db;Hd;Lk;Cl;Kl;Md;Kg;Mc;Ic;Jl;Hk;Fc;Kj;Bf;Hc;Fm;Lc;Cc;Ai;Jj;Dh;Ea;Ef;Ke;Ee;Fl;Ak;Ja;Kb;Fh;Gk;Ec;Eh;If;Al;Lf;Je;Kh;Ca;Cm;Hl;Mm;Bg;Ib;Hh;Fd;Lh;Ka;Bh;Fj;Id;Ki;Le;Il;Bk;El;Fi;Gb;Kf;Cg;Af;Dm;Ei;Ce;Ej;Jf;Fk;Fb;Jc;Jg;Di;Bm;Ac;Mf;Hi;Fe;Eb;Jd;Ge;Aj;Hb;Lb;Dc;Ii;Ad;Mk;Dd;Fa;Ga;Bi;Bd;Ji;La;Jh;Kd;Fg;Dg;Gi;Kc;Hg;Gm;Ch;Ab;Am;Bl;Cf;Km;Ag;Mb;Bc;Ae;Cb;Em;Em;He;Bb;Ah;Ke;Mg;Ed;Ci;Dk;Df;Gg;Aj;Ie;Im;Da;Jm;Li;Eg;Lg;Mj;Kk;Ij;Ba;Dj;Be;Gj;Hm;Ca;Ig;Jb;Hj;Ma;Aa;Bi;Hf;Ca;Ek;Gf;Gm;Bj;Mi;Ih;Ia;Ke;Ag;Jd;Ha;Hj

```