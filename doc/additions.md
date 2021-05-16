# Additions


---
* [Lists](#lists)
* [Chars](#chars)
* [Square bracket accessor](#square-bracket-accessor)
* [Object notation](#object-notation)
* [Grouping accessor](#grouping-accessor)
---


## Lists
List declaration:
```lux
const list = [1, 2, 3, "hi!"];

print list;	// [ 1, 2, 3, hi! ]
```


## Chars
Basically, a char type.

Syntax:
```lux
const a_char = 'a';
```

It's sole purpose is to help with string modification:
```lux
const letters = chars("luxya 🤢");

letters[6] = '✨';

print from_chars(letters);	// luxya ✨
```
Read more about [`chars` here](./native_functions.md#chars), [here](#square-bracket-accessor), and about [`from_chars` here](./native_functions.md#from_chars).


## Square bracket accessor
Accessing lists:
```lux
const list = ['a', 'b', 'c', 'd'];

print list[2];	// c
```

Accessing strings:
```lux
const name = "Ola";

print name[0];	// O
```

You may think that accessing strings is not reliable, coz what if we have a multibyte `char`? Let's try it.
```lux
const emoji = "🥺";

print emoji[0];	// â
```
`â` is not what we expected, but that's the desired behaviour. Luxya deals with accessing chars by using their byte representation, so that you can expect an O(1) operation on every access.

But what if we want to access the emoji as a char? We use the [`chars`](./native_functions.md#chars) function!
```lux
const name = "luxya ✨";

const expanded = chars(name);

print expanded;	// [ l, u, x, y, a,  , ✨ ]

print expanded[6];	// ✨
```
Neat! What we get is a list of chars that we can now safely and reliably access!


## Object notation
Luxya supports a full object notation!

Declare an empty object:
```lux
const object = {};
```

Declare objects with key value pairs:
```lux
const some_value = 1;

const object = {
	name: "luxya ✨",
	"arbitrary key": "value!",	// you can use strings as keys!
	some_value,	// shorthand key-value notation
};

print object;	// { name: luxya ✨, arbitrary key: value!, some_value: 1 }
```
As you can see, the last key doesn't have a value. That's because luxya supports a shorthand key-value notation - it will automatically bind the value if the value of the same name is declared in scope!


## Grouping accessor
By using the grouping accessor, you can get and set properties on objects with any arbitrary key, that you couldn't do with just a dot notation (`object.key`).
```lux
const object = {};

const key = "1 2 3 4 key with spaces";

object.(key) = "value!";

print object.(key);	// value!
```

By combining this accessor with [`has`](./native_functions.md#has) and [`unset`](./native_functions.md#unset) functions, you can treat objects like a hashmap.

```lux
const map = {};

map.("arbitrary key") = "value!";

print has(map, "arbitrary key");	// true

unset(map, "arbitrary key");
print has(map, "arbitrary key");	// false
```
