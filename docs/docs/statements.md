## Function bindings

- Syntactic sugar for `let` binding with function expression
    - When not mutable and top level, auto formatting will rewrite as `func` style
    - When mutable or not top level, will be written as `let` binding style
- Mandatory arg types (not inferred in any way)
- Arg grouping (`a` below is also of type `int` just like `b`)
- Default values (for example, `b` = `1` below, but `a` has no default value)
    - When arg grouping is used, a default value can only be used with the last arg
    - A fresh copy of each default arg is used at each call site (as if passed explicitly)
- Body is made up of one or more statements or expressions
- Exits at either last expression or return statement (early return)
- POSSIBLE FUTURE: Allow omitting of arg type when default literal value used

```
func my_func(a, b: int = 1) -> int:
    a + b
end

# is equivalent to:

let my_func =
    func (a, b: int = 1) -> int:
        a + b
    end
```

## Variable bindings

- `let` = bind a name to an expression (immutable binding)
- `let mut` = bind a name to an expression (mutable binding)
    - This means the binding itself can be reassigned, not that the value is mutable
- Type is optional in almost all cases (inferred from expression)
- Multiple bindings separated with a comma
    - Type Alternatives: All have type, None have type, or last has type (arg groups)
    - If using arg grouping where last entry has type, then it applies to all
- POSSIBLE FUTURE: Each binding type can be done in a block
    - Would allow for simple expression binding only

```
let     abc               = 123 + add_me()
let mut def         : str = "test" + "ok"
let     test1, test2      = 1, 2

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
