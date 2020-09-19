## Struct (FUTURE)

- Basic structure/record type

```
struct ABC:
    test : int
    test2: str
end

impl ABC:
    func new(test: int, test2: str):
        let self.test = test
        let mut self.test2 = test2
    end
end
```

## Enums (FUTURE)

- tagged union
- always pass by value (types inside can be ref)
- uses type constructures to initialize
- enums do not create a new scope
- immutable
- str() method returns name of enum from type constructor (ie. 'Thursday', 'Some("test")')
- from_str() method returns an enum, if possible (only type constructors w/o args supported)

```
# Effectively an interface + 2 structs
enum Option[T]:
    Some(T)
    None
end

# Effectively an interface + 2 structs
enum Result[T]:
    Ok(T)
    Error(str)
end

# Compiled efficiently to integers
enum DayOfWeek:
    Sunday
    Monday
    Tuesday
    Wednesday
    Thursday
    Friday
    Saturday
end

# Compiled efficiently to integers
enum Suites:
    Hearts = 0
    Spades = 1
    Diamonds = 2
    Clubs = 3
end

let x                      = Some(1)
let dow: Option[DayOfWeek] = DayOfWeek.from_str("Thursday)  # require derive annotation?

impl DayOfWeek:
    func is_weekend() -> bool:
        self match:
            Saturday, Sunday -> true
            _                -> false
        end
    end
end
```

## Interfaces (FUTURE)

- "magic methods"? Maybe....

```
interface equal:
    __equal__(any) -> bool
end

interface bool:
    __bool__() -> bool
end

interface iterator[T]:
    next() -> result[T]
end

impl equal for DayOfWeek:
    func __equal__(a: any) -> bool:
        match self:
            dow: DayOfWeek -> self == dow
            _              -> false
        end
    end
end
```
