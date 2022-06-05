# PL/XML Syntax

Programs that do not respect this syntax may (and probably
will) still work, but with no guarantee.

## McKeeman Form

```
plxml
    program

program
    "<program name=" tag ">" functions main functions "</program>"

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
    ws "<function name=" tag ">" arguments body "</function>" ws

arguments
    ws "<arguments>" arguments "</arguments>" ws

_arguments
    ""
    argument _arguments

argument
    ws "<argument name=" tag "/>" ws

body
    ws "<body>" instructions "</body>" ws

instructions
    ws "" ws
    instruction instructions

instruction
    ws _instruction ws

_instruction
    value
    assign
    integer
    real
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
    handle

value
    "<value variable=" tag "/>"

assign
    "<assign variable=" tag ">" instruction "</assign>"

integer
    "<integer value=" tag "/>"
    "<integer>" instruction "</integer>"

real
    "<real value=" tag "/>"
    "<real>" instruction "</real>"

string
    "<string value=" tag "/>"
    "<string>" instruction "</string>"

array
    "<array>" instructions "</array>"
    "<array />"

add
    "<add>" instructions "</add>"

subtract
    "<subtract>" instruction instructions "</subtract>"

multiply
    "<multiply>" instructions "</multiply>"

divide
    "<divide>" instruction instructions "</divide>"

and
    "<and>" instruction instructions "</and>"

or
    "<or>" instruction instructions "</or>"

not
    "<not>" instruction "</not>"

equal
    "<equal>" instruction instruction "</equal>"

greater
    "<greater>" instruction instruction "</greater>"

lower
    "<lower>" instruction instruction "</lower>"

call
    "<call function=" tag ">" call_arguments "</call>"
    "<call>" instruction call_arguments ws "</call>"

call_arguments
    ws "<arguments>" instructions "</arguments>" ws

return
    "<return>" instruction "</return>"

if
    "<if>" instruction then "</if>"
    "<if>" instruction then else "</if>"

then
    ws "<then>" instructions "</then>" ws

else
    ws "<else>" instructions "</else>" ws

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

handle
    ws "<handle>" try catch "</handle>" ws

try
    ws "<try>" instructions "</try>" ws

catch
    ws "<catch variable=" tag ">" instructions "</catch>" ws
```