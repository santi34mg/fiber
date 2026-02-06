# Lowering to SSA

Here I will discuss how lowering an AST into an SSA-like IR should function for
fib.
This is not a formal document, this is for internal development documentation.

## Expressions

While lowering, when the compiler encounters an expression node, it should match
its variant to decide the instructions to emit.

Expressions can be of the following types:

- Binary (contain a left and right expression, as well as an operator)
- Unary (contain an expression and an operator)
- Literal (contain only a literal)
- Identifier (contains the name of the identifier as a string)
- Grouping (contains another expression)
- Call (contains a callee and arguments, which are an expression and a vector of
  expressions respectively)

### Literal expressions

If the compiler encouters a literal expression, it should emit an instruction
that denotes that value (remember SSA treats variables as values).

```
1
```

Has the following AST:

```
Expression(
    Literal(
        Integer(
            1,
        ),
    ),
)
```

This should get converted into:

```
x_0 = 1
```

### Binary expressions

While lowering, when the compiler encounters a binary expression it should
recursively evaluate its left and then right (for left associativity) branches.
Then it should join them using the operator (use pattern matching).

```
1 + 2
```

Should get converted into:

```
x_0 = 1
y_0 = 2
z_0 = x_0 + y_0
```

A more complex example might be:

```
1 + 2 + 3
```

```
Expression(
    Binary {
        left: Binary {
            left: Literal(
                Integer(
                    1,
                ),
            ),
            operator: Plus,
            right: Literal(
                Integer(
                    2,
                ),
            ),
        },
        operator: Plus,
        right: Literal(
            Integer(
                3,
            ),
        ),
    },
),
```

This should get converted to:

```
x_0 = 1
y_0 = 2
z_0 = x_0 + y_0
a_0 = 3
b_0 = z_0 + a_0
```

### Unary expressions

Recursively evaluate the expression and then emit instruction with operator.

Source:

```
!true
```

AST:

```
Expression(
    Unary {
        operator: Not,
        expression: Literal(
            Boolean(
                true,
            ),
        ),
    },
),
```

Gets converted to:

```
x_0 = true
y_0 = NOT x_0
```
