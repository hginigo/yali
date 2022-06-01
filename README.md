# yali
*Yet Another Lisp Interpreter*.

A small Lisp interpreter written in Rust, heavily inspired by the Scheme Standard. **WIP**

## How to build
Clone this repository and change your working directory to the cloned repository:
```bash
$ git clone https://github.com/hginigo/yali.git
$ cd yali/
```
Now to build the interpreter just run:

```bash
$ cargo build
```
Or to run it directly:

```bash
$ cargo run
```
*Note*: [Rust](https://www.rust-lang.org/) (latest stable) is assumed to be installed for building this project.

Now let's try it:
```scheme
> (+ 1 2)
3
```

## Features
### Definitions and variables

```scheme
> (define number 100)
100
> (define string "a string")
"a string"
> (define a-list '(1 2 3))
(1 2 3)
> (set! number (* number 2))
200
```

`define` and `set!` return the value of the variable, as opposed in Scheme which is undefined.

*Note*: By now, the definition of functions is done via lambda syntax (see below).

### Quotation

```scheme
> 'a
a
> (quote (1 2 3))
(1 2 3)
> ''(1 2 3)
(quote (1 2 3))
```

### Cons, pairs, dotted lists

```scheme
> (cons 'a '())
(a)
> (cons '(a) '(b c d))
((a) b c d)
> (cons 'a (cons 'b 'c))
(a b . c)
> (quote (a b c . d))
(a b c . d)
```

### Lambdas
By now only the following argument syntax is available

```scheme
> ((lambda x
|     (* x 3)) 3) ; a single argument 
9
> (define rev-sub
|   (lambda (x y) (- y x)))
<lambda>
> (rev-sub 7 10) ; a fixed number of arguments
3
```

### Inspect
Symbols and bindings can be inspected calling `(inspect)`.

*Note*: This is for debugging purposes and may be changed in the future.
