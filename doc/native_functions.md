# Native functions


Explanation of syntax used in this section:
- a function which accepts argument of any type and returns a string: `name(any) -> string`
- a function which accepts an object and a string, and returns a boolean: `name(object, string) -> boolean`
- a function which accepts number or string for an argument and doesn't return anything: `name(number | string)`
- a function which returns a list of chars: `name() -> list[char]`


---
* [str](#str)
* [typeof](#typeof)
* [number](#number)
* [len](#len)
* [chars](#chars)
* [push](#push)
* [extend](#extend)
* [from_chars](#from_chars)
* [deep_copy](#deep_copy)
* [is_nan](#is_nan)
* [floor](#floor)
* [ceil](#ceil)
* [has](#has)
* [unset](#unset)
* [read](#read)
---


## str
Signature: `str(any) -> string`

You can use `str` to convert any value to a string.


## number
Signature: `number(number | string | char) -> number`

`number` converts a value to a number. The resulting number can be `NaN`. To test if a number is `NaN`, use [`is_nan`](#is-nan)

```lux
print number("1234");	// 1234
print number('a');	// NaN
```


## typeof
Signature: `typeof(any) -> string`

`typeof` returns a string representation of the first argument

```lux
print typeof(true);	// boolean
print typeof("luxya ✨");	// string
print typeof(2136);	// number
```


## len
Signature: `len(string | list[any]) -> number`

`len` returns the length of a string (in bytes) or a list


## chars
Signature: `chars(string) -> list[char]`

`chars` returns a list of chars in a string

```lux
const name = "luxya ✨";

print chars(name);	// [ l, u, x, y, a,  , ✨ ];
```

You can find example usages of `chars` [here](./additions.md#chars) and [here](./additions.md#square-bracket-accessor).


## from_chars
Signature: `from_chars(list[char]) -> string`

`from_chars` accepts a list of chars and returns a string


## push
Signature: `push(list[any], any) -> list[any]`

`push` can be used to push any value to the end of a list


## extend
Signature: `extend(list[any], list[any]) -> list[any]`

`extend` concatenates two lists


## deep_copy
Signature: `deep_copy(any) -> any`

`deep_copy` performs a deep copy of an object passed


## is_nan
Signature: `is_nan(number) -> any`

`is_nan` tells you if a number is `NaN`


## floor
Signature: `floor(number) -> number`

`floor` floors a number


## ceil
Signature: `ceil(number) -> number`

`floor` ceils a number, lol


## has
Signature: `has(object | list | string, any) -> number`

`has` tests if the first argument includes the second one

In case of an object, it tests if the first argument contains a string representation of the second argument as a key.

In case of a list, it tests if the list contains the second argument.

In case of a string it tests if the first argument contains a string representation of the second argument as a substring.


## unset
Signature: `unset(object, string) -> any`

`unset` removes an entry from an object under a provided key and returns the removed value. If there wasn't any value under the provided key, it returns `nil`.


## read
Signature: `read(string | nil) -> string`

`read` prints the provided string (or prints nothing if `nil` is provided) and waits for the user input, returning it
