IDENTIFIER ::= [A-Za-z_][A-Za-z_0-9]*

INT ::= [+-]?[0-9]+ | [+-]?0[xX][0-9A-Fa-f]* | [+-]?0b[0-1]*

FLOAT ::= [+-]?[0-9]+.[0-9]*(e[+-]?[0-9]+)?

STRING ::= // trivial

CHAR ::= // trivial



PROGRAM ::= BLOCK* Initialization [ IDENTIFIER* ]; Run [ IDENTIFIER* ];

BLOCK ::= RESOURCE | COMPONENT | SYSTEM

RESOURCE ::= Resource IDENTIFIER STRUCT

COMPONENT ::= Component IDENTIFIER STRUCT

STRUCT ::= { DECLARATION;* }

DECLARATION ::= const? TYPE IDENTIFIER

TYPE ::= Entity | System | void | int | float | uint | char | string | bool | TYPE[INT] | TYPE * | IDENTIFIER

SYSTEM ::= System IDENTIFIER (FILTERS) BLOCK

FILTER ::= IDENTIFIER, DECLARATION,* ,?

FILTERS ::= FILTER;* FILTER?

BLOCK ::= { EXPR*; EXPR? }

EXPR ::= DECLARATION =EXPR? | while (EXPR) BLOCK | break | continue | if (EXPR) BLOCK | if (EXPR) BLOCK else BLOCK | query (FILTERS) BLOCK | for (EXPR; EXPR; EXPR) | EXPR + EXPR | EXPR * EXPR | EXPR / EXPR | EXPR - EXPR | EXPR >> EXPR | EXPR << EXPR | EXPR ** EXPR | EXPR & EXPR | EXPR && EXPR | EXPR '|' EXPR | EXPR '||' EXPR | EXPR ^ EXPR | *EXPR | +EXPR | -EXPR | ~EXPR | !EXPR | &EXPR | BLOCK