# Expert system

## Why ?

This project is part of our school (42) cursus. It aims to create a solver for
simple boolean algebra. (https://cdn.intra.42.fr/pdf/pdf/6852/en.subject.pdf)

## How

The executable is made to be launched with a config file passed as arguement.

Usage:

`./expert-system config_file`

## Requirement

This project use the 1.39.0 version of Rust, but newer version of the language should work too.  
To install Rust: https://www.rust-lang.org/tools/install  
To install the 1.39.0 or Stable: https://doc.rust-lang.org/nightly/edition-guide/rust-2018/rustup-for-managing-rust-versions.html  

## Compilation

`cargo build --release` should compile the project and put the executable
in `./target/release`

### Configuration file

The configuration file has 3 main parts. 

- Rules definitions
- Facts assignations
- Queries

#### Rules definitions

The rules definition use the following syntax : `FACT OPERATOR FACT => FACT`
The possibles operators are : `+` (and), `^`(xor) and `|` (or).
ANDs operators (`+`) are allowed in the right side of the rule.
Parenthesis may be used.
The `!` (not) operator may be used to negate a fact or a operation group
(in front of parenthesis)

The `+` (and) and `^` (xor) have a higher priority over the `|`(or)

Example: 

```
A + B | (A ^ C) => D
```

#### Facts assignations

The Facts assignations start with `=`.  
It list all the facts that are to be set on true at the beginning of the computation.  

Example:

```
=ABD
```

#### Queries

Queries are used to ask `expert-system` the value of variables. A config file
can have multiple queries at different places.

Example:

```
?ABD
```

by Francis (GrandChaman) Le Roy and Guilhem (agt_smith) Smith
