# PL/XML Syntax

Programs that do not respect this syntax may (and probably
will) still work, but with no guarantee.

## McKeeman Form

```
plxml
    program

program
    "<program=" tag ">" functions main functions "</program>"

tag
    '"' characters '"'

characters
    ''
    character characters

character
    '0020' . '10FFFF' - '"' - '0027' - '<' - '>' - '&'
    '&' escapes ';'

ws
    ""
    '0020' ws
    '000A' ws
    '000D' ws
    '0009' ws

escapes
    "quot"
    "apos"
    "lt"
    "gt"
    "amp"

main
    ws "<main>" instructions "</main>" ws

functions
    ""
    function functions

function
    ws "<function name=" tag ">" ws "<arguments>" arguments "</arguments>" ws "<body>" instructions "</body>" ws "</function>" ws

arguments
    ""
    argument arguments

argument
    ws "<argument name=" tag "/>" ws

instructions
    ws "" ws
    instruction instructions

instruction
    ws _instruction ws

_instruction
    value
    assign
    integer
    float
    string
    array
    add
    subtract
    multiply
    divide
    and
    or
    not
    equal
    greater
    lower
    call
    return
    if
    for
    each
    while

value
    "<value variable=" tag "/>"

assign
    "<assign variable=" tag ">" instruction "</assign>"

integer
    "<integer value=" tag "/>"
    "<integer>" instruction "</integer>"

float
    "<float value=" tag "/>"
    "<float>" instruction "</float>"

string
    "<string value=" tag "/>"
    "<string>" instruction "</string>"

array
    "<array>" instructions "</array>"

add
    "<add>" instructions "</add>"

subtract
    "<subtract>" instructions "</subtract>"

multiply
    "<multiply>" instructions "</multiply>"

divide
    "<divide>" instructions "</divide>"

and
    "<and>" instructions "</and>"

or
    "<or>" instructions "</or>"

not
    "<not>" instruction "</not>"

equal
    "<equal>" instruction instruction "</equal>"

greater
    "<greater>" instruction instruction "</greater>"

lower
    "<lower>" instruction instruction "</lower>"

call
    "<call function=" tag ">" ws "<arguments>" instructions "</arguments>" ws "</call>"
    "<call>" instruction "<arguments>" instructions "</arguments>" ws "</call>"

return
    "<return>" instruction "</return>"

if
    "<if>" ws instruction ws "<then>" instructions "</then>" ws "</if>"
    "<if>" ws instruction ws "<then>" instructions "</then>" ws "<else>" instructions "</else>" ws "</if>"

each
    "<each variable=" tag ">" instruction do "</each>"

while
    "<while>" instruction do "</while>"

for
    "<for variable=" tag ">" ws from to step do "</for>"

from
    ws "<from>" instruction "</from>" ws

to
    ws "<to>" instruction "</to>" ws

step
    ws "<step>" instruction "</step>" ws

do
    ws "<do>" instructions "</do>" ws
```