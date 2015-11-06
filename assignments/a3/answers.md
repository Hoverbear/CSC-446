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

We note that $E[A+B] = E[A] + E[B]$ and $V[A+B] = V[A] + V[B]$.

$$ E[Y] = \sum_{i=1}^n E[X_i] = \sum_{i=1}^n \eta = n\eta $$
$$ V[Y] = \sum_{i=1}^n V[X_i] = \sum_{i=1}^n \sigma^2 = n\sigma^2$$

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

Using $M/M/1$ model:

$$ \rho = \frac{ \lambda_1 }{ \mu_1 } = \frac{ \frac{1}{6} \frac{cus}{min} }{ \frac{1}{2} \frac{cus}{min} } = \frac{1}{3} $$
$$ L= \frac{\rho}{1-\rho} = \frac{ \frac{1}{3} }{ 1 - \frac{1}{3} } = \frac{1}{2} $$
$$ L_Q = \frac{ \rho^2 }{ 1-\rho } = \frac{ (\frac{1}{3})^2 }{ 1- \frac{1}{3} } = \frac{1}{6} $$
$$ \omega = \frac{1}{ \mu(1-\rho) } = \frac{ 1 }{ \frac{1}{2}(1 - \frac{1}{3}) } = 3 $$
$$ \omega_Q = \frac{ \rho }{ \mu(1-\rho) } = \frac{ \frac{1}{3} }{ \frac{1}{2} (1 - \frac{1}{3}) } = 1 $$

### What are the performance metrics for Security-Check?

Using $M/M/1$ model with $\mu_s = \frac{5}{8}$:

$$ \rho = \frac{ \lambda_1 + \lambda_2 }{ \mu_1 } = \frac{ \frac{1}{6} + \frac{2}{6} }{ \frac{5}{8} } = \frac{4}{5} $$
$$ L= \frac{\rho}{1-\rho} = \frac{ \frac{4}{5} }{ 1-\frac{4}{5} } = 4 $$
$$ L_Q = \frac{ \rho^2 }{ 1-\rho } = \frac{ (\frac{4}{5})^2 }{ 1- \frac{4}{5} } = \frac{16}{5} $$
$$ \omega = \frac{1}{ \mu_s(1-\rho) } = \frac{ 1 }{ \frac{5}{8}(1-\frac{4}{5}) } = 8 $$
$$ \omega_Q = \frac{ \rho }{ \mu_s(1-\rho) } = \frac{ \frac{4}{5} }{ \frac{5}{8} (1- \frac{4}{5})} = \frac{32}{5} $$

### What is the total average time spent by Terminal 1 passengers in the system?

The total time spent by Terminal 1 passengers will be $\omega_1 + \omega_2 = 3+8 = 11$

### What is the total average number of Terminal 1 passengers in the system?

The total average number of Terminal One passengers in the system will be:

$$ L_1 + \frac{\lambda_1}{\lambda_1 +\lambda_s}L_S = \frac{1}{2} + \frac{\frac{1}{6}}{ \frac{1}{6}+\frac{2}{6} }(4) = \frac{11}{6} $$

### What is the average time spent by Terminal 2 passengers in the system?

The total time spent by Terminal 2 passengers will be equal to $\omega_s$ since through that and do not receive a priority.

$$ \omega_s = 8 $$

## Question 4

It's important to note that due to the wording of the question **we are only to simulate the security check queue**, not the entire system.

$$ L = \lambda \omega $$
$$ \omega_Q = \omega - \frac{1}{\mu} $$
$$ L_Q = \frac{\rho^2}{1-\rho} $$

| Seed | $\frac{1}{\lambda}$ | $\frac{1}{\mu}$ | $\rho$ | $\omega$ | $L$ | $\omega_Q$ | $L_Q$ |
|------|---------------------|-----------------|--------|----------|-----|------------|-------|
| 1    | 2.00154             | 1.58419         | 0.7913 | 10.8852  | 5.4384 | 9.3010  | 3.0002 |
| 2    | 2.02436             | 1.65779         | 0.8188 | 8.02259  | 3.9630 | 6.3648  | 3.6999 |
| 3    | 1.98914             | 1.57126         | 0.7899 | 6.87971  | 3.4586 | 5.3084  | 2.9697 |
| 4    | 1.99502             | 1.58535         | 0.7947 | 7.91235  | 3.9660 | 6.3270  | 3.0762 |
| 5    | 1.98116             | 1.60859         | 0.8120 | 7.42176  | 3.7461 | 5.8131  | 3.5071 |
|------|---------------------|-----------------|--------|----------|--------|---------|--------|
|      | 1.9982              | 1.6014          | .80134 | 8.2243   | 4.1144 | 6.6228  | 3.2506 |

## Question 5

Provided:

* Arrival Rate: $\lambda = 30 \text{per hr} = \frac{1}{2} \text{per minute} = \frac{1}{120} \text{per second}$
* Mean: $\frac{1}{\mu} = 90 s$
* Deviation: $\sigma = 90 s$
* Variance: $\sigma^2 = (90 s)^2$

Determining:

* $\mu = \frac{1}{90 s}$
* Utilization: $\rho = \frac{ \lambda }{ \mu } = \frac{ \frac{1}{120} \text{per second} }{ \frac{1}{90 s} } = \frac{3}{4}$
* $L_Q = \frac{ \rho^2(1+\sigma^2\mu^2) }{ 2(1-\rho) } = \frac{ (\frac{3}{4})^2(1+90^2(\frac{1}{90})^2)  }{ 2(1-\frac{3}{4}) } = \frac{9}{4} \text{ planes}$
* $\omega_Q = \frac{ \lambda(\frac{1}{\mu^2} + \sigma^2) }{ 2(1-\rho) } = \frac{ \frac{1}{120} (\frac{1}{\frac{1}{90^2}} + 90^2) }{ 2(1-\frac{3}{4}) } = 270 s$

At $\frac{25}{18}$ USD per second, the average cost is $270(\frac{25}{18}) = 375$ USD.

## Question 6

For an $M/M/1$ queues with a $\lambda$ arrival rate:

$$ \rho = \frac{\lambda}{\mu} $$
$$ L = \frac{\lambda}{\mu-\lambda} = \frac{\rho}{1-\rho} $$
$$ L_Q = \frac{\lambda^2}{\mu(\mu-\lambda)} = \frac{\rho^2}{1-\rho} $$
$$ \omega = \frac{1}{\mu-\lambda} = \frac{\rho}{1-\rho} $$
$$ \omega_Q = \frac{\mu}{\mu-\lambda} = \frac{\rho}{\mu(1-\rho)} $$

For two:

$$ L_T = L_1 + L_2 $$
$$ L_{Q,T} = L_{Q,1} + L_{Q_2} $$
$$ \omega_T = \omega_1 + \omega_2 $$
$$ \omega_{Q,T} = \omega_{Q,1} + \omega_{Q,2} $$

For an $M/M/2$ queue with $2\lambda$ arrival rate, and $c=2$:

$$ \rho = \frac{2\lambda}{c\mu} = \frac{\lambda}{\mu} $$
$$ L = c\rho + \frac{ (c\rho)^{c+1}P_0 }{ c(c!)(1-\rho)^2 } = c\rho + \frac{ \rho P(L(\infty) \geq c) }{ 1-\rho } $$
$$ L_Q = 2\lambda\omega_Q $$
$$ \omega = \frac{L}{2\lambda} $$
$$ \omega_Q = \omega - \frac{1}{\mu} $$

Unfortunately the equations significantly differ, however we can note that the utilization remains the same between the two scenarios. This means that there are no catastrophic effects.

We'll test using $\lambda = 4$ and $\mu = 8$:

In the two $M/M/1$ case we calculate:

$$ \rho = \frac{\lambda}{\mu} = \frac{4}{8} = \frac{1}{2} $$
$$ L = \frac{\rho}{1-\rho} = \frac{ \frac{1}{2} }{ 1 - \frac{1}{2} } = 1 $$
$$ \omega = \frac{\rho}{1-\rho} = \frac{\frac{1}{2}}{1-\frac{1}{2}} = 1 $$

In the $M/M/2$ case we calculate:

$$ \rho = \frac{2\lambda}{2\mu} = \frac{1}{2} $$
$$ P_0 = (\sum_{n=0}^{c-1} \frac{(c\rho)^n}{n!} + (c\rho)^c(\frac{1}{c!})(\frac{1}{1-\rho}))^{-1} = (\frac{(2\frac{1}{2})^1}{1!} + (2\frac{1}{2})^2(\frac{1}{2!})(\frac{1}{1-\frac{1}{2}}))^{-1} = \frac{1}{2} $$
$$ P(L(\infty) \geq c) = \frac{(c\rho)^cP_0}{c!(1-\rho)} = \frac{ (2\frac{1}{2})^2\frac{1}{2} }{ 2!(1-\frac{1}{2}) } = \frac{1}{2} $$
$$ L = c\rho + \frac{ \rho P(L(\infty) \geq c) }{ 1-\rho } = 2\frac{1}{2} + \frac{ \frac{1}{2} * \frac{1}{2} }{ 1-\frac{1}{2} } = \frac{3}{2} $$
$$ \omega = \frac{\frac{3}{2}}{2*4} = \frac{3}{16} $$

By those accounts, the $M/M/2$ style queue helps reduce the individual wait times, but does not otherwise contribute to the efficiency or capability of the system.

## Question 7

This is an $M/M/4/3/\infty$ system with the following probabilities:

| p  | Type                     |
|----|--------------------------|
| .2 | 3 min (Rinse)            |
| .7 | 7 min (Wash, Rinse)      |
| .1 | 12 min (Was, Rinse, Wax) |

The arrival rate, $\lambda = 34 \frac{cars}{hr} = \frac{34}{60} \frac{cars}{min}$. We further note that if a car arrives at the car wash when there is $>n$ cars there ($n$ being the number of stalls) it will be turned away.

$\therefore$ We seek to calculate $P_3$ (for the three stall case) and $P_4$ (for the 4 stall case).

First, $P_3$ in the three stall case:

$$ \mu = a = \frac{1}{ .2(3) + .7(7) + .1(12) } = \frac{10}{67} $$
$$ \rho = \frac{\lambda}{c\mu} = \frac{ \frac{34}{60} }{ 1(\frac{10}{67}) } = 3.796 $$
$$ P_0 = \frac{1}{ 1 + \sum_{n=1}^c \frac{a^n}{n!} + \frac{a}{c!} \sum_{n=c+1}^N \rho^{n-c} } = \frac{1}{ 1+\frac{ \frac{10}{67} }{1!} + \frac{ (\frac{10}{67})^1 }{1!}((\frac{10}{67})^{2-1} + (\frac{10}{67})^{3-1}) } = 0.8511... $$
$$ P_3 = \frac{a^NP_0}{c!c^{N-c}} = \frac{ (\frac{10}{67})^3 (0.8511) }{ 1!1^{3-1} } = 0.00289 $$

$\therefore$ there is a .2% chance for a customer to arrive and be dropped from the line.

Then, $P_4$ in the four stall case:

$$ \mu = a = \frac{1}{ .2(3) + .7(7) + .1(12) } = \frac{10}{67} $$
$$ \rho = \frac{\lambda}{c\mu} = \frac{ \frac{34}{60} }{ 1(\frac{10}{67}) } = 3.796 $$
$$ P_0 = \frac{1}{ 1 + \sum_{n=1}^c \frac{a^n}{n!} + \frac{a}{c!} \sum_{n=c+1}^N \rho^{n-c} } = \frac{1}{ 1+\frac{ \frac{10}{67}) }{1!} + \frac{ (\frac{10}{67})^1 }{1!}((\frac{10}{67})^{2-1} + (\frac{10}{67})^{3-1} (\frac{10}{67})^{4-1}) } = 0.8535... $$
$$ P_3 = \frac{a^NP_0}{c!c^{N-c}} = \frac{ (\frac{10}{67})^4 (0.8535) }{ 1!1^{4-1} } = 0.0004235 $$

$\therefore$ there is a .04% chance for a customer to arrive and be dropped from the line.

## Question 8

This is $M/M/1$ queues chained together with:

$$ \lambda = 1 \text{ per hr} $$
$$ \mu = 4 \text{ per hr} $$

Then the **mean** ($E[X]$) and **variance** ($V[X]$) are:

$$ E[X] = \sum_{i=1}^3 \frac{1}{\mu} = \sum_{i=1}^3 \frac{1}{4} = \frac{3}{4} = \frac{1}{\mu_t}$$
$$ V[X] = \sum_{i=1}^3 V[x_i] = \sum_{i=1}^3 \frac{1}{\mu^2} = 3(\frac{1}{4^2}) = \frac{3}{16} $$

This **is not an exponential distribution**, we use $M/G/1$ equations:

The **average number of delayed persons** ($L_Q$), with $\rho = \frac{\lambda}{\mu_t} = \frac{1}{\frac{4}{3}} = \frac{3}{4}$ is:

$$ L_Q = \frac{ \rho^2 (1 + \sigma^2 \mu^2) }{ 2(1-\rho) } = \frac{ (\frac{3}{4})^2(1+\frac{3}{16}(\frac{4}{3})^2) }{ 2(1-\frac{3}{4}) } = \frac{3}{2} $$

The **total mean time a customer spends in the system**, ($\omega$):

$$ \omega = \omega_Q + \frac{1}{\mu} = \frac{L_Q}{\lambda} + \frac{1}{\mu} = \frac{3}{2} + \frac{1}{ \frac{4}{3} } = \frac{9}{4} $$

## Question 9

$$ .9(x) = \lambda $$
$$ x = \frac{5}{.9} = 5.5 $$

The **long run expected delay** will be $\omega_Q$ since "delay" only accounts for time not spent processing.

Time spent waiting in the repair portion, an $M/M/2/\infty/\infty$ system, $c=2$, $\lambda=5.5$:

$$ \rho_{\text{Repair}} = \frac{\lambda}{\mu} = \frac{5.5}{2(3)} = \frac{5.5}{6} $$
$$ P_0 = \frac{1}{ \sum_{n=0}^{c-1} (\frac{(c\rho)^n}{n!}) + ((c\rho)^c(\frac{1}{c!})(\frac{1}{1-\rho})) } = \frac{1}{ (\frac{(2\frac{5.5}{6})^1}{1!}) + ((2\frac{5.5}{6})^2(\frac{1}{2!})(\frac{1}{1-\frac{5.5}{6}})) } = \frac{1}{22} $$
$$ P(L(\infty) \geq c) = \frac{ (\frac{\lambda}{\mu})^cP_0 }{ c!(1-\rho) } = \frac{ (\frac{5.5}{3})^(2) \frac{1}{22} }{ 2!(1-\frac{5.5}{6}) } = \frac{11}{12} $$
$$ L = c\rho + \frac{\rho P(L(\infty) \geq c)}{ 1-\rho } = 2(\frac{5.5}{6}) + \frac{\frac{5.5}{6} * \frac{11}{12} }{ 1 - \frac{5.5}{6} } = 11.9 $$
$$ \omega_{Q, \text{Repair}} = \omega - \frac{1}{\mu} = \frac{L}{\mu} - \frac{1}{\mu} = \frac{11.9}{3} - \frac{1}{3} = 3.63 $$

Time spent waiting in the inspection portion, an $M/M/1$ queue with $\lambda = 5.5$, $\mu=8$:

$$ \rho = \frac{\lambda}{\mu} = \frac{5.5}{8} $$
$$ \omega_{Q, \text{Inspection}} = \frac{\rho}{\mu(1-\rho)} = \frac{\frac{5.5}{8}}{8(1-\frac{5.5}{8})} = \frac{11}{40} $$

The total is $\omega_{Q, \text{Repair}}+\omega_{Q, \text{Inspection}} = 3.63 + \frac{11}{40} = 3.905$.

The **maximum arrival rate** that the system can handle without adding personnel would be $6$, however since there is a 10% feedback we must calculate $\lambda=6*.9=5.4$.
