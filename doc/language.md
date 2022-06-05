# PL/XML: The Handbook

## Introduction

As do many ideas, PL/XML sprang from the need
to do something extremely useless just for fun,
to see how it would turn out. The original premise
was a programming language based on XML syntax.
Its name came from (PL/SQL)[https://en.wikipedia.org/wiki/PL/SQL],
some kind of torture device for IT students.

In order not to make this project completely useless,
I wrote the original interpreter in Rust (WIIR!)
to get better acquainted with the language.

The result is a dynamically-typed procedural language,
which is provably Turing-complete since I was able to
write a blazing slow (Brainfuck interpreter)[../sample/bf.pl.xml]
with it. All the convenient aspects are overshadowed
by the utter agony that manually writing XML is.

## Quick guide

```xml
<program name="primer">
    <function name="my-print">
        <arguments>
            <argument name="the-text" />
        </arguments>
        <body>
            <call function="print-line">
                <arguments>
                    <value variable="the-text" />
                </arguments>
            </call>
        </body>
    </function>
    <main>
        <assign variable="text">
            <string value="Hello, world!">
        </assign>
        <call function="my-print">
            <arguments>
                <value variable="text" />
            </arguments>
        </call>
    </main>
</program>
```

This slightly over-engineered hello world program
contains some basics of PL/XML, such as program structure,
variable assignment and retrieval, instanciation, and function
definition and calls.

### Program structure

Every PL/XML program should be wrapped in a `program`
node specifying its name. This program must contain a
`main` node that will be executed first, and can define
a set of functions using `function` nodes. Inside `main`
and the function `body` nodes is actual code that will be
sequentially executed.

### Values

PL/XML has a few value types. The first two are the signed numeric
`integer` and `real` types, which have no precision guarantee.
Another type is the usual character `string`, which may or may not
support Unicode. The `array` type is a generic iterable collection
of any value, including arrays. Functions are values as well, and
as such can be (and technically are) stored in variables.

Integer, Real and String values can be instanciated by using the
eponymous node with a `value` attribute. For instance:

```xml
<integer value="1" />
<real value="2.5" />
<string value="hello!" />
```

Value nodes `integer`, `real`, and `string` can also be used
to cast a value to another type. For instance, a `string` value
can be parsed into a `real`, and a `real` value can be rounded down
by casting it to an `integer`.

```xml
<real>
    <integer>
        <real value="2.5" />
    </integer>
</real>
```

Arrays can be initialized empty or with contained elements. Array
manipulation is performed through [standard library](stl.md) functions.

```xml
<array />

<array>
    <integer value="0" />
    <integer value="1" />
    <integer value="2" />
    <integer value="3" />
    <real value="3.14" />
</array>
```

When boolean-like values are needed, all values
are considered truthy, except the integer 0.

### Variable manipulation

A value can be assigned to and retrieved from a variable.
Variables are dynamically-typed, meaning you can assign any
type to any variable, no matter its previous type.

To assign a value to a variable, use an `assign` node with a
`variable` attribute specifying the name of the variable, and
add a child node containing any value-returning node, such as
`string`.

```xml
<assign variable="my-variable">
    <string value="hello!" />
</assign>
```

To retrieve a value, use a `value` node with the same
`variable` attribute.

```xml
<value variable="my-variable" />
```

Variables have some sort of scoping which is function body-bound:
there is a global scope containing standard and user-defined
functions from which local scopes inherit.

### Function calls

A `call` node is used to call functions. Function arguments
are passed as child nodes to a `arguments` node. The short
syntax uses the `function` attribute to specify the function
to call.

```xml
<call function="my-print">
    <arguments>
        <string value="text" />
    </arguments>
</call>
```

Functions are values that can also be stored and retrieved
through variables. Thus exists a longer syntax allowing dynamic
calls, without the `function` attribute but putting the function
value as a child of the `call` node.

```xml
<call>
    <value variable="my-print" />
    <arguments>
        <string value="text" />
    </arguments>
</call>
```

You can find a variety of utility functions in the
[standard library](stl.md).

### Function definition

Functions are defined at the top level, as child nodes to the
`program` node. The `name` attribute specifies the function name
to use when called. They have a `arguments` node, defining to which
local variables arguments will be assigned, in order of `argument` nodes,
and a `body` node containing the code that will be executed when the function
is called.

```xml
<function name="my-print">
    <arguments>
        <argument name="the-text" />
    </arguments>
    <body>
        <call function="print-line">
            <arguments>
                <value variable="the-text" />
            </arguments>
        </call>
    </body>
</function>
```

This function, named "my-print" takes one argument, called "the-text".
It uses this value to call the standard library "print-line" function.

Functions can return values. Wrap a value in a `return` node to use it as
a return value for the function. Subsequent code will not be executed, and
the caller can use the `call` node as any other value.

```xml
<function name="sum-plus-two">
    <arguments>
        <argument name="number1" />
        <argument name="number2" />
    </arguments>
    <body>
        <return>
            <add>
                <value variable="number1" />
                <value variable="number2" />
                <integer value="2" />
            </add>
        </return>
    </body>
</function>
```

This function takes two arguments and adds them together,
adding two to the sum, and returns the result.

### Built-in operations

The previous example uses an `add` node to sum integer values.
PL/XML has multiple usual arithmetic and logic operators to
manipulate values, used directly as nodes containing them.

Only compatible values will be used together. Integers will
automatically be promoted to reals if needed.

`add` and `multiply` both take any number of number arguments and will
compute their sum or product. `add` can also be used to concatenate
string values.

```xml
<add>
    <integer value="9" />
    <integer value="33" />
</add>

<add>
    <string value="hello, " />
    <string value="world!" />
</add>

<multiply>
    <integer value="6" />
    <real value="7" />
</multiply>
```

`subtract` and `divide` take at least one numeric argument, which will be
subtracted from or divided using subsequent arguments.

```xml
<subtract>
    <integer value="51" />
    <integer value="9" />
</subtract>

<divide>
    <integer value="126" />
    <integer value="3" />
</divide>
```

`and` and `or` also take at least one argument, and will chain their
corresponding logic operation on all arguments.

```xml
<and>
    <integer value="1" />
    <string value="yes" />
</and>

<or>
    <integer value="0" />
    <string value="no" />
</or>
```

`not` takes exactly one argument, and will give a truthy value
(the integer 1) if the argument is falsy (the integer 0), and
a falsy value otherwise.

```xml
<not>
    <string value="make me falsy" />
</not>
```

`equal`, `greater`, and `lower` all take exactly two arguments,
and will give a truthy value if the first is respectively
equal to, greater than, or lower than the second, and a falsy
value otherwise.

```xml
<equal>
    <integer value="5" />
    <integer value="5" />
</equal>

<greater>
    <integer value="4" />
    <integer value="2" />
</greater>

<lower>
    <integer value="11" />
    <integer value="16" />
</lower>
```

### Control structures

As in many imperative languages, control structures are
used to manipulate the flow of code execution. The first
one is the `if` structure. Its first child is the value checked
for truthyness, after which a `then` block contains the code
to execute if it is truthy, otherwise the code contained in the
optional `else` block will be executed.

```xml
<if>
    <value variable="my-condition" />
    <then>
        <call function="print-line">
            <arguments>
                <string value="truthy" />
            </arguments>
        </call>
    </then>
    <else>
        <call function="print-line">
            <arguments>
                <string value="falsy" />
            </arguments>
        </call>
    </else>
</if>
```

Three other structures give access to loops. `while` loops
contain the condition to check, which will be executed at the beginning
of each loop turn, and a `do` node containing the code to execute.

```xml
<while>
    <integer value="1" />
    <do>
        <call function="print-line">
            <arguments>
                <string value="forever!" />
            </arguments>
        </call>
    </do>
</while>
```

The `for` loop takes `from`, `to`, and `step` child nodes, which should
evaluate to integer values. Code contained in the `do` child node
will be executed with a variable whose name is specified in the `variable`
attribute on the `for` node containing the current iteration value.

```xml
<for variable="i">
    <from><integer value="0" /></from>
    <to><integer value="10" /></to>
    <step><integer value="1" /></from>
    <do>
        <call function="print-line">
            <arguments>
                <add>
                    <string value="iteration #" />
                    <string>
                        <value variable="i" />
                    </string>
                </add>
            </arguments>
        </call>
    </do>
</for>
```

Finally, the `each` loop iterates over an array, assigning its values
in order to the specified `variable`.

```xml
<each variable="v">
    <value variable="my-array" />
    <do>
        <call function="print-line">
            <arguments>
                <add>
                    <string value="value = " />
                    <string>
                        <value variable="v" />
                    </string>
                </add>
            </arguments>
        </call>
    </do>
</each>
```

### Error handling

Some standard library functions or language nodes may raise
errors during execution. In a `handle` node, errors can be
caught inside a `try` node to use them in a `catch` node, which
will only be executed if an error was raised.

```xml
<handle>
    <try>
        <divide>
            <integer value="1" />
            <integer value="0" />
        </divide>
    </try>
    <catch variable="error">
        <call function="print-line">
            <arguments>
                <add>
                    <string value="eroor caught = " />
                    <value variable="error" />
                </add>
            </arguments>
        </call>
    </catch>
</handle>
```