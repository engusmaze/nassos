# Simplang

It is an experimental programming language not intended for general use in programming.

## Examples

### Hello world

###### hello-world.slg

```py
define 0        # Define variable 0 with default value false
define 1        # Define variable 1 with default value false
set 1 true      # Set value of variable 1 to true

ascii 1 0 0 1 0 0 0     # H
ascii 1 1 0 0 1 0 1     # e
ascii 1 1 0 1 1 0 0     # l
ascii 1 1 0 1 1 0 0     # l
ascii 1 1 0 1 1 1 1     # o
ascii 0 1 0 0 0 0 0     #
ascii 1 1 1 0 1 1 1     # w
ascii 1 1 0 1 1 1 1     # o
ascii 1 1 1 0 0 1 0     # r
ascii 1 1 0 1 1 0 0     # l
ascii 1 1 0 0 1 0 0     # d
ascii 0 1 0 0 0 0 1     # !

newline                 # Print \n
```

### Counting to 15 in binary

###### 4bit-counter.slg

```py
# 4-bit number "buffer"
define a0
define a1
define a2
define a3

# Some useful variables
define bit
define bittmp
define skip

label again     # Label to use with goto

# Hard to explain magic
set bit true

copy bit to bittmp
and a0 bittmp bit
xor a0 bittmp a0

copy bit to bittmp
and a1 bittmp bit
xor a1 bittmp a1

copy bit to bittmp
and a2 bittmp bit
xor a2 bittmp a2

copy bit to bittmp
and a3 bittmp bit
xor a3 bittmp a3

# Just print our number bit by bit
print a0
print a1
print a2
print a3
newline

# Check if current number is the max 4-bit number
set skip true
and skip a0 skip
and skip a1 skip
and skip a2 skip
and skip a3 skip
not skip skip

# If it is not the max number then go to beggining
goto again if skip
```

### Instructions

`define {variable}`

`set {variable} {value}`

`copy {variable} to {variable}`

`and {variable a} {variable b} {output variable}`

`or {variable a} {variable b} {output variable}`

`xor {variable a} {variable b} {output variable}`

`not {variable} {output variable}`

`print {variable}`

`ascii {variable} {variable} {variable} {variable} {variable} {variable} {variable}`

`newline`

`label {label name}`

`goto {label}`
