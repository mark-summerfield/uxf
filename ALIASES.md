= Aliases

uxf branch alias

== BNF

    VALUETYPE    ::= KEYTYPE | 'bool' | 'real' | 'list' | 'map' | 'table' | IDENFIFIER # IDENFIFIER is an alias or a table name
    ALIAS        ::= '@' COMMENT? OWS IDENFIFIER (RWS (VALUETYPE | ALIASTYPE))+ # IDENFIFIER is the alias
    ALIASTYPE    ::= '[' OWS (VALUETYPE | ALIASTYPE) OWS ']' '{' OWS KEYTYPE (RWS (VALUETYPE | ALIASTYPE))? '}'

== Additional system import

    !aliases

which is equivalent to:

    uxf 1
    @key bytes date datetime int str'
    @number int real
    @scalar bool key real
    @smap {key scalar}

== Examples

**Motivating examples: if there aren't any then this isn't worth doing!**

    uxf 1
    !aliases
    =Point x:number y:number
    [
      [number 1 2.3 4 5.6 7 8.9]
      (Point  1 2.3 4 5.6 7 8.9)
    ]
