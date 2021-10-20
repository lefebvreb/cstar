IDENTIFIER ::= [A-Za-z_][A-Za-z_0-9]*

INT ::= [+-]?[0-9]+ | [+-]?0[xX][0-9A-Fa-f]* | [+-]?0b[0-1]*

FLOAT ::= [+-]?[0-9]+.[0-9]*(e[+-]?[0-9]+)?

STRING ::= // todo



PROGRAM ::= BLOCK* Initialization [ IDENTIFIER* ]; Run [ IDENTIFIER* ];

BLOCK ::= RESOURCE | COMPONENT | SYSTEM

RESOURCE ::= Resource IDENTIFIER STRUCT

COMPONENT ::= Component IDENTIFIER STRUCT

STRUCT ::= { DECLARATION;* }

DECLARATION ::= const? TYPE IDENTIFIER

TYPE ::= int | float | char | string | TYPE[INT] | TYPE *

SYSTEM ::= System IDENTIFIER ( FILTER;* FILTER? ) { STATEMENT* }

FILTER ::= IDENTIFIER, DECLARATION,* ,?

STATEMENT ::= // todo

EXPR ::= // todo