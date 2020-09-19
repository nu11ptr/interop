## Value Types

### Boolean

- Type name: **bool**
- Size: **typically 8 bits, but depends on platform**

```
true
false
```

### Integer

- Type name: **int, uint**
- Size: **at least 31 bits (varies by platform?)**

```
1234
1234u   # future
```

#### Alternate formats, not repeated for others but can be combined (FUTURE)

```
0x0f0f      # hex
0b0101      # binary
0o0707      # octal
1_000_000   # underscores for grouping
```

### Long Integer (FUTURE)

- Type name: **long, ulong**
- Size: **at least 63 bits (varies by platform)**

```
1234L
1234uL
```

### Exact sized integers (FUTURE)

- Type name: **int8, uint8, byte (alias for uint8), int16, uint16, int32, uint32, int64, uint64**
- Size: **8 to 64 bits (based on suffix)**

```
1i8
1u8
1b  # equiv to 1u8
1i16
1u16
1i32
1u32
1i64
1u64
```

_FUTURE: Char - char (4 bytes - default encoding is UTF8)_

```
'a'
'\uFFFFFFFF'   # Unicode literal char
'\xFF'         # hex char
'\o77'         # octal char
'\b01010101'   # binary char
```

### String

- Type name: **str**
- Stored as a byte array
- Immutable
- Ends with a null 0 for C compatibility but is not used otherwise in length calculation

```
"abc"       # Std set of encodings
``"abc"``   # future - contains any char except double backtick
```

### Unit

- Type name: **()**
- Effectively an empty tuple
- Only one value

```
() # Future
```

### Tuple

- Type name: **(int,int)**
  - _# of types listed based on tuple arity_
- Immutable - changing it returns a new tuple
- Implemented using something like an array (but passed by value)

```
(1,)
(1, 2)
```

## Reference Type Expressions

### Array (FUTURE)

- Type name: **array[int]**
- Fixed length, not resizable
- Mutable

```
[|1, 2, 3|]
```

### List (FUTURE)

- Type name: **list[int]**
- Has capacity and length
- Appendable up to capacity and then auto-grows
- Implemented using an array
- Can be indexed - O(n) and sliced
- Python-like index operations
- Mutable
- TODO: Doesn't address length/capacity allocation

```
[1, 2, 3]

let abc                = [1, 2, 3] # if all elements are same type, no need specify type
let abc: list[addable] = ["a", 1]  # if aren't all same type, list the interface they share (defaults to list[any])
```

### Set (FUTURE)

- Type name: **set[int]**
- Implemented using a map (or modified map)
- No guaranteed iteration order
- Mutable
- TODO: Doesn't address capacity allocation

```
{|1, 2, 3|}

let abc               = {|1, 2, 3|} # if all elements are same type, no need specify type
let abc: set[addable] = {|"a", 1|}  # if aren't all same type, list the interface they share (defaults to list[any])
```

### Map (FUTURE)

- Type name: **map[str, int]**
- No guaranteed iteration order
- Mutable
- TODO: Doesn't address capacity allocation

```
{1 -> 2, 3 -> 4}

let abc                    = {1 -> 'a', 3 -> 'b'} # if all elements are same type (all keys/all values), no need specify type
let abc: map[int, addable] = {1 -> "a", 2 -> 1}  # if aren't all same type, list the interface they share (defaults to map[int, any] in this case)
```

**TODO: Missing - floating point? arbitrary length decimal? bigint?**