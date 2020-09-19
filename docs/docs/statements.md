## Function bindings

- Mandatory arg types (not inferred in any way)
- Arg grouping ('a' below is also of type int just like 'b')
- Default values (for example, '1' below
  - A fresh copy of each default arg is used at each call site
- Body is made up of one or more statements or expressions
- Exits at either last expression or return statement

```
func my_func(a, b: int = 1) -> str:
    "test"
end
```

## Variable binding

- **let** = bind a name to an expression (immutable binding)
- **let mut** = bind a name to an expression (mutable binding)
- Type is optional in almost all cases (inferred from expression)
- FUTURE: Each binding type can be done in a block

```
let     abc               = 123 + add_me()
let mut def         : str = "test" + "ok"
let     test1, test2      = 1, 2           # future - rhs is a tuple, it is unpacked into test1/test2

# POSSIBLE FUTURE
let:
    test12345      = 123
    tester   : int = 456
    abc            = 789
end
```

## Assignments

- Bindings assigned by **let mut** can be reassigned

```
def = 456   # def must have been bound by 'let mut'
```

## Break, Continue, Return

```
func xyz() -> int:
    return 1
end

let y = loop:
    if x then:
        continue
    else:
        break 1
    end
end
```
