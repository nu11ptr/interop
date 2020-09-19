## If/Then/Else Expression

- FUTURE: In addition to bool expressions, if/elif will accept any type that implements the **Bool** interface
  - As a safety precaution, type **unit** should not implement **Bool** to prevent assignments in if expressions

```
# With blocks
let abc =
    if test == "123" then:
        let abc = 1000
        abc
    else if test == "456" then:
        let xyz = 456
        xyz
    else:
        let def = 7
        def
    end

# Without blocks
let abc2 = if test == "123" then 1000 else if test == "456" then 456 else 7
```

## Function Expression

- Similar to function bindings, minus the binding name
- FUTURE: Consider a shorter/simpler one liner syntax for inline functions

```
let abc =
    func (a, b: int = 1) -> str:
        "test"
    end

test(param1 = 123,
     param2 =
         func (a, b: int = 1) -> str:
             "test"
         end)

# FUTURE
func (a, b) -> a + b   # single expression only, return type derived from expression?
```

## Prefix Expressions

```
(1)
-1
not 1   # FUTURE: rhs can be any type that implements Bool interface
```

## Infix Expressions

```
1 + 2
1 - 2
1 * 2
1 / 2
1 mod 2

1 == 2
1 != 2
1 >= 2
1 <= 2
1 > 2
1 < 2

5 in [1, 2, 3]
5 not in [1, 2, 3]
1 and 2            # FUTURE: lhs/rhs can be any type that implements Bool interface
1 or 2             # FUTURE: lhs/rhs can be any type that implements Bool interface

# Bitwise
250 ^ 32    # xor
255 & 10    # and
16 | 48     # or
~ 16        # not (prefix expr)

# Call
myname(1, 2, 3)
myname(1, num = 2, age = 3)     # Can specify args names (once you specify one, the rest must also)

# FUTURE: Indexing (arrays, lists, tuples, and maps)
# FUTURE: Anything that supports the right interface
myname[0]     # first element
myname[-1]    # last element, python-style
myname[10:15] # slice

# FUTURE: Qualified names
mymod.test
myobj.attr
```

## Comprehensions (POSSIBLE FUTURE)

- TODO: Syntax could be hard to parse

```
(x <- for x in range(start: 1, end: 10) if x > 0)    # iterator
[x <- for x in range(start: 1, end: 10) if x > 0]    # list
{x <- for x in range(start: 1, end: 10) if x > 0}    # map
{|x <- for x in range(start: 1, end: 10) if x > 0|}    # set
```

## Match (FUTURE)

- Basic pattern matching
- Typical bind syntax create bindings for matched types
- 'as' binds entire expression

```
func abc(a Any) -> result[int]:
    match a:
        Some(x: int) -> x
        b: int       -> b
        _ as y       -> y    # Catch all
    end
end

match abc:
    1, 2 -> true
    _    -> false
end
```

## Try/Catch (POSSIBLE FUTURE)

- Not exceptions
- Used with things that implement a special interface

```
func abc(a, b: int) -> result[int]:
    func do_ret(err: error) -> error:
        println(err)
        err
    end

    # single expression version
    # err is default - specified for illustration
    let z =
        try some_op_that_can_fail() catch as err:
            do_ret(err)
        end
end


func abc(a, b: int) -> result[int]:
    func do_ret(err: error) -> error:
        println(err)
        err
    end

    # multi expression version
    let abc =
        try:
            z <- some_op_that_can_fail() # <- syntax only allowed in try block
            print(z)                     # Normal statements allowed in block too
            x <- some_other_op()
            x + z
        catch as err:  # err is default - specified for illustration
            do_ret(err)
        end
end
```

## For

- Works with any type that implements the **Iterator[T]** interface
- TODO: Loop labels needed??? (could be tricky with it being an expression)
  - alternative: break break?
- FUTURE: Consider making this an expression
  - break EXPR works as an early return
  - Otherwise, last expression is returned

```
for x in xyz:
    if false then break else continue
end

# FUTURE: What this could look like in the future???
let z =
    for x in xyz:
        if x == 100 then:
            break x + 23
        end
    else:
        x
    end
```

## While

- Simple while block

```
while true:
    if false then break else continue
end

# FUTURE: What this could look like in the future???
let z =
    while x > 0:
        if x == 100 then:
            break x + 23
        end
        x += 1
    else:
        x
    end
```

## Loop

- Infinite loop
- Effectively Sugar for 'while true'

```
let x =
    loop:
        if false then:
            break 1
        else:
            continue
        end
    end
```

## With (FUTURE)

- Similiar to Python 'with'
- Can be used with any type implementing the 'context' interface
- Special method called on entry and on exit from the block
- If decide to make expression, returns last expression

```
let x =
    with mytype(123) as a:
        a.do_stuff()
        123
    end
```

## Coroutines (POSSIBLE FUTURE)

- if has optional expression behind it, returns it

```
# Coroutine producing a "lazy list"
for xyz(x: int) -> coroutine[int, ()]:
    for y in range(x,100):
        yield y * 10
    end
end


# Coroutine
for xyz() -> coroutine[(), int]:
    loop:
        let y := yield
        println(y)
    end
end
```

## Async/Await

TODO
