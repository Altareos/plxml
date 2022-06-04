# Standard Library

The PL/XML Standard Library allows access to system functions
such as input/output, file access and array manipulation.

## Table of Contents

- [PRINT](#print)
- [PRINT-LINE](#print-line)
- [INPUT](#input)
- [STRING-SPLIT](#string-split)
- [ARRAY-SET](#array-set)
- [ARRAY-PUSH](#array-push)
- [ARRAY-POP](#array-pop)
- [ARRAY-GET](#array-get)
- [ARRAY-LENGTH](#array-length)
- [TO-ASCII](#to-ascii)
- [FROM-ASCII](#from-ascii)
- [GET-ARGS](#get-args)
- [WRITE-FILE](#write-file)
- [READ-FILE](#read-file)

## Functions

### PRINT

Writes a string as-is to the standard output

#### Arguments

- `string` value to print

#### Returns

Nothing

#### Minimal example

```xml
<call function="print">
    <arguments>
        <string value="hello world!&#xD;&#xA;" />
    </arguments>
</call>
```

### PRINT-LINE

Writes a string to the standard output, appending a new line

#### Arguments

- `string` value to print

#### Returns

Nothing

#### Minimal example

```xml
<call function="print-line">
    <arguments>
        <string value="hello world!" />
    </arguments>
</call>
```

### INPUT

Reads from the standard input

#### Arguments

Nothing

#### Returns

`string` value read

#### Minimal example

```xml
<call function="input">
    <arguments />
</call>
```

### STRING-SPLIT

Splits a string into a vector of single-character strings

#### Arguments

- `string` value to split

#### Returns

`array` value of strings

#### Minimal examples

```xml
<call function="string-split">
    <arguments>
        <string value="abcdef" />
        <string value="" />
    </arguments>
</call>
```
```xml
<call function="string-split">
    <arguments>
        <string value="id,name,type,value" />
        <string value="," />
    </arguments>
</call>
```

### ARRAY-SET

Sets a value at a specific index of an array.

#### Arguments

- `array` to update
- `integer` index
- `any` value to set

#### Returns

Nothing

#### Minimal example

```xml
<assign variable="arr">
    <array>
        <string value="hello">
    </array>
</assign>
<call function="array-set">
    <arguments>
        <value variable="arr" />
        <integer value="0" />
        <string value="world">
    </arguments>
</call>
```

### ARRAY-PUSH

Pushes a value at the end of an array

#### Arguments

- `array` to update
- `any` value to push

#### Returns

Nothing

#### Minimal example

```xml
<assign variable="arr">
    <array>
        <string value="hello">
    </array>
</assign>
<call function="array-push">
    <arguments>
        <value variable="arr" />
        <string value="world">
    </arguments>
</call>
```

### ARRAY-POP

Removes and returns the value at the end of an array

#### Arguments

- `array` to update

#### Returns

`any` value

#### Minimal example

```xml
<assign variable="arr">
    <array>
        <string value="hello">
    </array>
</assign>
<call function="array-pop">
    <arguments>
        <value variable="arr" />
    </arguments>
</call>
```

### ARRAY-GET

Returns the value at index of an array

#### Arguments

- `array` to query
- `integer` index

#### Returns

`any` value

#### Minimal example

```xml
<assign variable="arr">
    <array>
        <string value="hello">
    </array>
</assign>
<call function="array-get">
    <arguments>
        <value variable="arr" />
        <integer value="0" />
    </arguments>
</call>
```

### ARRAY-LENGTH

Returns the length of an array

#### Arguments

- `array` to query

#### Returns

`integer` length

#### Minimal example

```xml
<assign variable="arr">
    <array>
        <string value="hello">
    </array>
</assign>
<call function="array-length">
    <arguments>
        <value variable="arr" />
    </arguments>
</call>
```

### TO-ASCII

Converts an integer value into an ASCII character string

#### Arguments

- `integer` to convert

#### Returns

`string` corresponding ASCII character

#### Minimal example

```xml
<call function="to-ascii">
    <arguments>
        <integer value="65" />
    </arguments>
</call>
```

### FROM-ASCII

Converts a one-character string into its ASCII integer value

#### Arguments

- `string` character to convert

#### Returns

`integer` ASCII value

#### Minimal example

```xml
<call function="from-ascii">
    <arguments>
        <string value="a" />
    </arguments>
</call>
```

### GET-ARGS

Returns an array of arguments passed to the program

#### Arguments

Nothing

#### Returns

`array` of strings

#### Minimal example

```xml
<call function="get-args">
    <arguments />
</call>
```

### WRITE-FILE

Writes a string to a file, optionally appending

#### Arguments

- `string` filename
- `string` to write
- `any` falsy to replace, truthy to append

#### Returns

Nothing

#### Minimal example

```xml
<call function="write-file">
    <arguments>
        <string value="file.txt" />
        <string value="Hello world!" />
        <integer value="0" />
    </arguments>
</call>
```

### READ-FILE

Reads a file into a string

#### Arguments

- `string` filename

#### Returns

- `string` file contents

#### Minimal example

```xml
<call function="read-file">
    <arguments>
        <string value="file.txt" />
    </arguments>
</call>
```