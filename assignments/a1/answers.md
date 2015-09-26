---
title: CSC 446 A1
author: Andrew Hobden (V00788452)
---

## Question 1

Noting:

| Toss  | Type      | Time Between Arrivals | Processing Time |
|------:|:---------:|:---------------------:|:----------------|
| Heads |  Home     | 6                     | 2               |
| Tails |  Business | 2                     | 4               |

Sequence: THTTTHTHHTHHHTHTTTHT

The simulation table is:

| Cust # |Coin | Interarrival | Arrival | Service | Service Begins | Waiting | Service Ends | Total | Idle |
|:-------|:----|:-------------|:--------|:--------|:---------------|:--------|:-------------|:------|:-----|
| 1      | T   | 2            | 2       | 4       | 2              | 0       | 6            | 4     | 0    |
| 2      | H   | 6            | 8       | 2       | 8              | 0       | 10           | 2     | 2    |
| 3      | T   | 2            | 10      | 4       | 10             | 0       | 14           | 4     | 0    |
| 4      | T   | 2            | 12      | 4       | 14             | 2       | 18           | 6     | 0    |
| 5      | T   | 2            | 14      | 4       | 18             | 4       | 22           | 8     | 0    |
| 6      | H   | 6            | 20      | 2       | 22             | 2       | 24           | 4     | 0    |
| 7      | T   | 2            | 22      | 4       | 24             | 2       | 28           | 6     | 0    |
| 8      | H   | 6            | 28      | 2       | 28             | 0       | 30           | 2     | 0    |
| 9      | H   | 6            | 34      | 2       | 34             | 0       | 36           | 2     | 4    |
| 10     | T   | 2            | 36      | 4       | 36             | 0       | 40           | 4     | 0    |
| 11     | H   | 6            | 42      | 2       | 42             | 0       | 44           | 2     | 2    |
| 12     | H   | 6            | 48      | 2       | 48             | 0       | 50           | 2     | 4    |
| 13     | H   | 6            | 54      | 2       | 54             | 0       | 56           | 2     | 4    |
| 14     | T   | 2            | 56      | 4       | 56             | 0       | 60           | 4     | 0    |
| 15     | H   | 6            | 62      | 2       | 62             | 0       | 64           | 2     | 2    |
| 16     | T   | 2            | 64      | 4       | 64             | 0       | 68           | 4     | 0    |
| 17     | T   | 2            | 66      | 4       | 68             | 2       | 72           | 6     | 0    |
| 18     | T   | 2            | 68      | 4       | 72             | 4       | 76           | 8     | 0    |
| 19     | H   | 6            | 74      | 2       | 76             | 2       | 78           | 4     | 0    |
| 20     | T   | 2            | 76      | 4       | 78             | 2       | 80           | 4     | 0    |



The event list is:

| Clock  | Customer # | Arr/Dep |
|-------:|:-----------|:--------|
| 2      | 1          | Arrival |
| 6      | 1          | Depart  |
| 8      | 2          | Arrival |
| 10     | 2          | Depart  |
| 10     | 3          | Arrival |
| 12     | 4          | Arrival |
| 14     | 3          | Depart  |
| 14     | 5          | Arrival |
| 18     | 4          | Depart  |
| 20     | 6          | Arrival |
| 22     | 5          | Depart  |
| 22     | 7          | Arrival |
| 24     | 6          | Depart  |
| 28     | 7          | Depart  |
| 28     | 8          | Arrival |
| 30     | 8          | Depart  |
| 34     | 9          | Arrival |
| 36     | 9          | Depart  |
| 36     | 10         | Arrival |
| 40     | 10         | Depart  |
| 42     | 11         | Arrival |
| 44     | 11         | Depart  |
| 48     | 12         | Arrival |
| 50     | 12         | Depart  |
| 54     | 13         | Arrival |
| 56     | 13         | Depart  |
| 56     | 14         | Arrival |
| 60     | 14         | Depart  |
| 62     | 15         | Arrival |
| 64     | 15         | Depart  |
| 64     | 16         | Arrival |
| 66     | 17         | Arrival |
| 68     | 16         | Depart  |
| 68     | 18         | Arrival |
| 72     | 17         | Depart  |
| 74     | 19         | Arrival |
| 76     | 18         | Depart  |
| 76     | 20         | Arrival |
| 78     | 19         | Depart  |
| 80     | 20         | Depart  |

## Question 2

### Q2.a

$$ \frac{4+2+4+6+8+4+6+2+2+4+2+2+2+4+2+4+6+8+4+4}{20} = \frac{80}{20} = 4 $$

$\therefore$ The Average Service Time is 4 seconds.

### Q2.b

$$ \frac{11*2 + 9*6}{20} = \frac{76}{20} = 3.8 $$

$\therefore$ The Average Interarrival Time is 3.8 seconds.

### Q2.c

Where $\rho$ is the simulated utilization, $\lambda$ is the simulated arrival rate, and $\mu$ is the service rate.

$$ \rho = \frac{\lambda}{\mu} $$

$$ \lambda = \frac{1}{\text{Avg. Interarrival Time}} = \frac{1}{3.8} $$

$$ \mu = \frac{1}{\text{Avg. Service Rate}} = \frac{1}{4} $$

$\therefore$ The simulated utilization is:

$$\rho = \frac{ \frac{1}{3.8} }{ \frac{1}{4} } = 1.052$$

## Question 3
