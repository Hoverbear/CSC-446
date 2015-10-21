---
title: CSC 446 A2
author: Andrew Hobden (V00788452)
---

## Question 1

### Q1.a

$$
f(x) = \left\{
        \begin{array}{ll}
            x    & \quad 0 \leq x < \frac{c}{2} \\
            -x+c & \quad \frac{c}{2} \leq x \leq c \\
            0    & \quad x < 0, x > c
        \end{array}
    \right.
$$

### Q1.b

$$
F(x) = \left\{
        \begin{array}{ll}
            \int_{0}^x (x) & \quad 0 \leq x < \frac{c}{2} \\
            \int_{0}^{\frac{c}{2}} (x) + \int_{\frac{c}{2}}^x (-x+c) & \quad \frac{c}{2} \leq x \leq c \\
            0    & \quad x < 0, x > c
        \end{array}
    \right.
$$

$$
F(x) = \left\{
        \begin{array}{ll}
            \frac{x^2}{2} & \quad 0 \leq x < \frac{c}{2} \\
            \frac{ \frac{c}{2}^2 }{2} + \frac{1}{2}( \frac{c}{2} - x )( \frac{c}{2} + x - 2c ) & \quad \frac{c}{2} \leq x \leq c \\
            0    & \quad x < 0, x > c
        \end{array}
    \right.
$$

### Q1.c

$$ F(x=c) = 1 $$

$$ 1 = \frac{ \frac{c}{2}^2 }{2} + \frac{1}{2}( \frac{c}{2} - c )( \frac{c}{2} + c - 2c ) $$

$$ 1 = \frac{ c^2 }{ 8 } + \frac { c^2 }{ 8 } $$

$$ \sqrt{4} = c $$

$$ c = 2 $$

### Q1.d

First, since we now know $c$, simplifying the CDF:

$$
F(x) = \left\{
        \begin{array}{ll}
            \frac{x^2}{2} & \quad 0 \leq x < 1 \\
            \frac{1}{2} + \frac{1}{2}( 1 - x )( x - 3 ) & \quad 1 \leq x \leq 2 \\
            0    & \quad x < 0, x > 2
        \end{array}
    \right.
$$

Now:

$$ P(0.5 \leq X \leq 1.5) = F(1.5) - F(0.5) $$

$$ F(1.5) = \frac{1}{2} + \frac{1}{2}( 1 - 1.5 )( 1.5 - 3 ) = 0.875 $$

$$ F(0.5) = \frac{(0.5)^2}{2} = 0.125 $$

$$ P(0.5 \leq X \leq 1.5) = 0.875 - 0.125 = 0.75 $$
