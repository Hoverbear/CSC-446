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

### Q1.e

$$ E[X] = \int_\infty^\infty x f(x) dx $$

Decomposing the piecewise function, recalling $c=2$:

$$ E[X] = (\int_{-\infty}^0 x(0) dx) + (\int_0^{1} x(x) dx) + (\int_{1}^2 x(-x+2) dx) $$

$$ E[X] = 0 + (\frac{1^3}{3} - \frac{0^3}{3}) + ((2^2-\frac{2^3}{3}) - (1^2-\frac{1^3}{3})) = 1 $$

Finding $V[X]$:

$$ V[X] = E(X^2) - \mu $$

$$ V[X] = (\int_{-\infty}^0 x^2(0) dx) + (\int_0^{1} x^2(x) dx) + (\int_{1}^2 x^2(-x+2) dx) - 1^2 $$

$$ V[X] = 0 + (\frac{1^4}{4} - \frac{0^4}{4}) + ((\frac{2(2)^3}{3} - \frac{(2)^4}{4}) - (\frac{2(1)^3}{3} - \frac{(1)^4}{4})) - 1^2 = \frac{1}{6} $$

### Q1.f

$$
f(x) = \left\{
        \begin{array}{ll}
            \frac{x}{2}    & \quad 0 \leq x < c \\
            -\frac{x}{2}+c & \quad c \leq x \leq 2c \\
            0    & \quad x < 0, x > 2c
        \end{array}
    \right.
$$

## Question 2

<!-- TODO -->

## Question 3

### What is the Average Service Time at the Security-Check?

Given:

$$ \lambda_1 = 10 \frac{cus}{hr} = \frac{1}{6} \frac{cus}{min}$$
$$ \lambda_2 = 20 \frac{cus}{hr} = \frac{2}{6} \frac{cus}{min}$$
$$ \frac{1}{\mu_1} = 2 \frac{min}{cus} $$
$$ \mu_1 = \frac{1}{2} \frac{cus}{min} $$
$$ \rho_s = \frac{\lambda}{\mu} = .80 $$

Determining:

$$ \rho_s = \frac{ \lambda_1 + \lambda_2 }{ \mu_s } $$
$$ .80 = \frac{ \frac{1}{6} + \frac{2}{6} }{ \mu_s } $$

$$ \mu_s = \frac{ \frac{1}{6} + \frac{2}{6} }{ .80 } $$
$$ \mu_s = \frac{5}{8} $$

$$ \frac{1}{\mu_s} = 1.6 \frac{min}{cus} $$

### What is the performance metrics for Terminal 1 Check-in?

<!-- TODO -->

$$ \rho = \frac{ \lambda_1 }{ \mu_1 } = \frac{ \frac{1}{6} \frac{cus}{min} }{ \frac{1}{2} \frac{cus}{min} } = \frac{1}{3} $$
$$ L = \lambda_1 \omega_1 $$
$$ L_Q = $$
$$ \omega = $$
$$ \omega_Q = $$

## Question 4

<!-- TODO -->

## Question 5

Arrival Rate: $\lambda = 30 \text{per hr}$

Mean: $\frac{1}{\mu} = 90 s$

Deviation: $\sigma = 90 s$

Variance: $\sigma^2 = (90 s)^2$
