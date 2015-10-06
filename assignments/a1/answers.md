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

$$ \frac{11*4 + 9*2}{20} = \frac{80}{20} = 3.1 $$

$\therefore$ The Average Service Time is 3.1 seconds.

### Q2.b

$$ \frac{11*2 + 9*6}{20} = \frac{76}{20} = 3.8 $$

$\therefore$ The Average Interarrival Time is 3.8 seconds.

### Q2.c

Where $\rho$ is the simulated utilization, $\lambda$ is the simulated arrival rate, and $\mu$ is the service time.

$$ \rho = \frac{\lambda}{\mu} $$

$$ \mu = \frac{1}{\text{Avg. Service Rate}} = \frac{1}{3.1} $$

$$ \lambda = \frac{1}{\text{Avg. Interarrival Time}} = \frac{1}{3.8} $$

$\therefore$ The simulated utilization is:

$$ \rho = \frac{ \frac{1}{3.8} }{ \frac{1}{3.1} } = \frac{31}{38} $$

### Q2.d

Theoretical chance of `Heads`: 50%

Theoretical chance of `Tails`: 50%

Theoretical Service Time:

$$ \frac{10*4 + 10*2}{20} = \frac{60}{20} = 3 $$

Theoretical Interarrival time:

$$ \frac{10*2 + 10*6}{20} = \frac{80}{20} = 4 $$

The theoretical server utilization is:

$$ \frac{ \frac{1}{4} }{ \frac{1}{3} } = \frac{3}{4} $$

There is a discrepancy, yes, it's because stochastic processes are random therefore only estimates.

### Q2.e

The average time a customer spends in the system is:

$$ \frac{ 4+2+4+6+8+4+6+2+2+4+2+2+2+4+2+4+6+8+4+4 }{ 20 } = 4 $$

### Q2.f

Average Service Time:

$$ \frac{2\rho + 4(1-\rho)}{20} $$

To compute the bounds of $\rho$ so that the system is stable we need to compute the $\rho$ value at $1.00$ utilization.

$$ \frac{ \frac{6\rho + 2(1-\rho)}{20} }{ \frac{2\rho + 4(1-\rho)}{20} } < 1.00 $$

Solving yields $\rho < \frac{1}{3}$.

## Question 3

**Since the spreadsheets provided are incompatible with LibreOffice the problem has been reimplemented in Rust. See `code/able-baker/src/main.rs`**

### Q3.a & Q3.b

Code is included within the archive.

### Q3.c

On a 100 customers:

```
Average Waiting Time: 0.29
Probability of Waiting: 0.22
Probability of Idle Server: 0.50199203187251
Average Service Time: 3.81
Average Time Between Arrivals: 2.242424242424242
Average Waiting time of those who wait: 1.3181818181818181
Average Time Caller Spends in System: 4.1
```

### Q3.c

Modified Policy:

```
Average Waiting Time: 0.225
Probability of Waiting: 0.17
Probability of Idle Server: 0.5231388329979879
Average Service Time: 3.705
Average Time Between Arrivals: 2.271356783919598
Average Waiting time of those who wait: 1.3235294117647058
Average Time Caller Spends in System: 3.93
```

Original:

```
Average Waiting Time: 0.225
Probability of Waiting: 0.16
Probability of Idle Server: 0.5766129032258065
Average Service Time: 3.555
Average Time Between Arrivals: 2.2663316582914574
Average Waiting time of those who wait: 1.40625
Average Time Caller Spends in System: 3.78
```

Code Diff for original policy:

```diff
--- a/assignments/a1/code/able-baker/src/main.rs
+++ b/assignments/a1/code/able-baker/src/main.rs
@@ -118,21 +118,7 @@ fn main() {
         // If both idle, choose randomly.
         // If one idle, assign to it.
         // If both busy, choose soonest available.
-        if (able.busy_until < clock) && (baker.busy_until < clock) {
-            let mut rng = rand::thread_rng();
-            let range = Range::new(0u64, 100);
-            let rand_val = range.ind_sample(&mut rng);
-            // No waiting.
-            caller.waiting_time = Some(0);
-            // There is an idle time!
-            if rand_val <= 50 {
-                able.idle_time = able.idle_time + (clock - able.busy_until);
-                able.serve(&mut clock, &mut caller);
-            } else {
-                baker.idle_time = baker.idle_time + (clock - baker.busy_until);
-                baker.serve(&mut clock, &mut caller);
-            }
-        } else if able.busy_until < clock {
+        if able.busy_until < clock {
             // No waiting.
             caller.waiting_time = Some(0);
             // There is an idle time for Able
```

The new policy is anecdotally better in these few simulations. This is because it more rapidly balances load across the two servers.

## Question 4

**This Java code is exceptionally strange.**

The diff for modifying the code to calculate average service and interarrival times is:

```diff
@@ -7,6 +7,10 @@ public static double Clock, MeanInterArrivalTime, MeanServiceTime, SIGMA, LastEv
 public static long  NumberOfCustomers, QueueLength, NumberInService,
         TotalCustomers, NumberOfDepartures, LongService;

+// Calculate the mean service and interarrival time.
+public static double TotalServiceTime = 0;
+public static double TotalInterArrivalTime = 0;
+
 public final static int arrival = 1;
 public final static int departure = 2;

@@ -66,7 +70,9 @@ public static void main(String argv[]) {
   if (MaxQueueLength < QueueLength) MaxQueueLength = QueueLength;

   // schedule the next arrival
-  Event next_arrival = new Event(arrival, Clock+exponential(stream, MeanInterArrivalTime));
+  double next_interarrival_time = exponential(stream, MeanInterArrivalTime);
+  TotalInterArrivalTime += next_interarrival_time;
+  Event next_arrival = new Event(arrival, Clock+next_interarrival_time);
   FutureEventList.enqueue( next_arrival );
   LastEventTime = Clock;
  }
@@ -75,6 +81,7 @@ public static void main(String argv[]) {
   double ServiceTime;
   // get the job at the head of the queue
   while (( ServiceTime = normal(stream, MeanServiceTime, SIGMA)) < 0 );
+  TotalServiceTime += ServiceTime;
   Event depart = new Event(departure,Clock+ServiceTime);
   FutureEventList.enqueue( depart );
   NumberInService = 1;
@@ -104,17 +111,17 @@ double PC4   = ((double)LongService)/TotalCustomers;


 System.out.println( "SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER ");
-System.out.println( "\tMEAN INTERARRIVAL TIME                         "
-	+ MeanInterArrivalTime );
-System.out.println( "\tMEAN SERVICE TIME                              "
-	+ MeanServiceTime );
+System.out.println( "\tMEAN INTERARRIVAL TIME                         "
+	+ (TotalInterArrivalTime / TotalCustomers) );
+System.out.println( "\tMEAN SERVICE TIME                              "
+	+ (TotalServiceTime / TotalCustomers) );
 System.out.println( "\tSTANDARD DEVIATION OF SERVICE TIMES            " + SIGMA );
 System.out.println( "\tNUMBER OF CUSTOMERS SERVED                     " + TotalCustomers );
```

### Q4.a

```java
MeanInterArrivalTime = 4.5;
// ...
Event next_arrival = new Event(arrival, Clock+exponential(stream, MeanInterArrivalTime));
```

$\therefore$ `4.5` and Exponential.

### Q4.b

```java
MeanServiceTime = 3.2;
// ...
while (( ServiceTime = normal(stream, MeanServiceTime, SIGMA)) < 0 );
```

$\therefore$ `3.2` and Normal.

### Q4.c

$$ \rho = \frac{ \frac{1}{4.5} }{ \frac{1}{3.2} } $$

$$ \rho = 0.711\bar{1} $$

### Q4.d

Seed of `1`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.474767351330928
	MEAN SERVICE TIME                              3.205465396384677
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     1000

	SERVER UTILIZATION                             0.7154468565718423
	MAXIMUM LINE LENGTH                            9.0
	AVERAGE RESPONSE TIME                          6.943036600826695  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.667
	SIMULATION RUNLENGTH                           4474.645676201413 MINUTES
	NUMBER OF DEPARTURES                           1000
```

Seed of `2`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.43090269543457
	MEAN SERVICE TIME                              3.1866357652352666
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     1000

	SERVER UTILIZATION                             0.71902369792077
	MAXIMUM LINE LENGTH                            14.0
	AVERAGE RESPONSE TIME                          8.947211232575318  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.694
	SIMULATION RUNLENGTH                           4431.892543250238 MINUTES
	NUMBER OF DEPARTURES                           1000
```

### Q4.e

Seed of `3`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.415666273123853
	MEAN SERVICE TIME                              3.213289831693823
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     5000

	SERVER UTILIZATION                             0.7275907905216983
	MAXIMUM LINE LENGTH                            15.0
	AVERAGE RESPONSE TIME                          7.867269519507706  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.6968
	SIMULATION RUNLENGTH                           22078.202627905997 MINUTES
	NUMBER OF DEPARTURES                           5000
```

Seed of `4`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.441362660932404
	MEAN SERVICE TIME                              3.2007940754753883
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     5000

	SERVER UTILIZATION                             0.7210881468715392
	MAXIMUM LINE LENGTH                            10.0
	AVERAGE RESPONSE TIME                          7.497442853219246  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.681
	SIMULATION RUNLENGTH                           22190.093430508758 MINUTES
	NUMBER OF DEPARTURES                           5000
```

### Q4.f

The specified outputs (mean inter-arrival time, mean service time, server utilization and mean response time) are not significantly affected by raising the amount of customers or changing the seed.

### Q4.g

```diff
- while (( ServiceTime = normal(stream, MeanServiceTime, SIMGA)) < 0 );
+ while (( ServiceTime = exponential(stream, MeanServiceTime)) < 0 );
```

Seed `5`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.447823890055081
	MEAN SERVICE TIME                              3.149403896391417
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     1000

	SERVER UTILIZATION                             0.7078971020757688
	MAXIMUM LINE LENGTH                            11.0
	AVERAGE RESPONSE TIME                          9.593773298308685  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.687
	SIMULATION RUNLENGTH                           4448.9571819921475 MINUTES
	NUMBER OF DEPARTURES                           1000
```

Seed `6`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.6600612521778
	MEAN SERVICE TIME                              3.0989400524232003
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     1000

	SERVER UTILIZATION                             0.666161570357085
	MAXIMUM LINE LENGTH                            10.0
	AVERAGE RESPONSE TIME                          8.260061297381522  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.66
	SIMULATION RUNLENGTH                           4650.332091699408 MINUTES
	NUMBER OF DEPARTURES                           1000
```

### Q4.h

**Assuming we still use the code from Q4.g.**

Seed `7`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.403470897484633
	MEAN SERVICE TIME                              3.15350165785368
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     5000

	SERVER UTILIZATION                             0.716259545501339
	MAXIMUM LINE LENGTH                            15.0
	AVERAGE RESPONSE TIME                          10.867609877240296  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.6978
	SIMULATION RUNLENGTH                           22012.236273517236 MINUTES
	NUMBER OF DEPARTURES                           5000
```

Seed `8`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.598245423933367
	MEAN SERVICE TIME                              3.1966901623828328
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     5000

	SERVER UTILIZATION                             0.6950843404794042
	MAXIMUM LINE LENGTH                            16.0
	AVERAGE RESPONSE TIME                          9.793335102067426  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.6812
	SIMULATION RUNLENGTH                           22992.105334684264 MINUTES
	NUMBER OF DEPARTURES                           5000
```

Seed `9`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.443506280798436
	MEAN SERVICE TIME                              3.2469910343643287
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     5000

	SERVER UTILIZATION                             0.7306452967151716
	MAXIMUM LINE LENGTH                            30.0
	AVERAGE RESPONSE TIME                          11.426843472500124  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.714
	SIMULATION RUNLENGTH                           22218.7191194186 MINUTES
	NUMBER OF DEPARTURES                           5000
```

Seed `10`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.398235311240714
	MEAN SERVICE TIME                              3.168403147687611
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     5000

	SERVER UTILIZATION                             0.7204563037404577
	MAXIMUM LINE LENGTH                            22.0
	AVERAGE RESPONSE TIME                          12.031856635584338  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.6928
	SIMULATION RUNLENGTH                           21988.583976479018 MINUTES
	NUMBER OF DEPARTURES                           5000

```

Seed `11`:

```
SINGLE SERVER QUEUE SIMULATION - GROCERY STORE CHECKOUT COUNTER
	MEAN INTERARRIVAL TIME                         4.462716023207889
	MEAN SERVICE TIME                              3.1813326702943683
	STANDARD DEVIATION OF SERVICE TIMES            0.6
	NUMBER OF CUSTOMERS SERVED                     5000

	SERVER UTILIZATION                             0.7127609636930727
	MAXIMUM LINE LENGTH                            18.0
	AVERAGE RESPONSE TIME                          10.523241687605557  MINUTES
	PROPORTION WHO SPEND FOUR
	 MINUTES OR MORE IN SYSTEM                     0.6882
	SIMULATION RUNLENGTH                           22311.158536622694 MINUTES
	NUMBER OF DEPARTURES                           5000
```

* Mean Interarrival:
    + Mean:

$$ \frac{ 4.403470897484633 + 4.598245423933367 + 4.443506280798436 + 4.398235311240714 + 4.462716023207889 }{ 5 } $$

$$ = 4.4612347873330078 $$

    + Variance:

$$ \frac{ (4.403470897484633 - 4.4612347873330078)^2 + (4.598245423933367 - 4.4612347873330078)^2 + (4.443506280798436 - 4.4612347873330078)^2 + (4.398235311240714 - 4.4612347873330078)^2 + (4.462716023207889 - 4.4612347873330078)^2 }{ 5 } $$

$$ = 0.00527880190072355068961864785736 $$

* Mean Service Time:
    + Mean: $3.18938373451656416$

    + Variance: $0.00103290259367070007$

* Server Utilization:
    + Mean: $0.71504129002588904$

    + Variance: $0.00013555425361533648$

* Mean Response Time:
    + Mean: $10.9285773549995482$

    + Variance: $0.58445666085304932742$

## Question 5

1 trial:

| Bin | Freq | Prob |
|-----|------|------|
| 3   | 8    | $\frac{8}{25}=.32$ |
| 6   | 11   | $\frac{11}{25}=.44$ |
| 10  | 6    | $\frac{6}{25}=.24$ |

5 trial:

| Bin | Freq              | Prob                  |
|-----|-------------------|-----------------------|
| 3   | 7+7+8+4+8=34      | $\frac{34}{125}=.272$ |
| 6   | 13+11+10+13+13=60 | $\frac{60}{125}=.480$ |
| 10  | 5+7+7+8+4=31      | $\frac{31}{125}=.248$ |

10 trial:

| Bin | Freq                            | Prob              |
|-----|---------------------------------|-------------------|
| 3   | 6+5+8+10+10+10+11+8+11+4=83     | $\frac{83}{250}=.332$ |
| 6   | 16+14+11+10+13+9+8+10+7+14=112  | $\frac{112}{250}=.448$ |
| 10  | 3+6+6+5+2+6+6+7+7+7=55          | $\frac{55}{250}=.22$ |

The simulation fits the intended distribution within reason. Excels terrible random number generator helps skew the results a bit.

## Question 6

### Q6.a

$$ \frac{ 1*0 + 4*1 + 11*2 + 35*3 + 60*4 + 47*5 + 22*6 + 18*7 + 0*8 + 2*9 + 0*10}{ 200 } = 4.41 $$

### Q6.b

$$ \frac{ 2*0 + 8*1 + 30*2 + 85*3 + 85*4 + 101*5 + 52*6 + 27*7 + 9*8 + 1*9 + 0*10 }{ 400 } = 4.375 $$

### Q6.c

$$ \frac{ 1*5 + 4*6 + 13*7 + 47*8 + 88*9 + 47*10 }{ 200 } = 8.79 $$

### Q6.d

$$ \frac{ 1*2 + 1*3 + 15*4 + 5*17 + 57*6 + 54*7 + 41*8 + 13*9 + 1*10 }{ 200 } = 6.625 $$

### Q6.e

Noting that the goal is **0.6** hits average.

Let $\rho_x = 700$.

### Q6.f

We can infer that:

* The average number of hits does not change if the sample size does not change.
* Small changes to one variable can lead to dramatic differences in results (Eg $c$ compared to $d$)
* To find extremes in average number of hits you need to work at extreme values.

## Question 7

**See `code/parameter/src/main.rs`**

### Q7.a

```
p hat: 0.296
Normalized estimation error: 0.013333333333333346
```

### Q7.b

```
p hat: 0.091
Normalized estimation error: 0.09000000000000008
```

### Q7.c

```
p hat: 0.001
Normalized estimation error: 0
```

### Q7.d

```
p hat: 0
Normalized estimation error: 1
```

### Q7.e

We can conclude that 1000 is a very small sample set when dealing with numbers this small. To reduce estimation error increase the sample size.

For example, setting $p=0.4$ and doing `10_000_000` trials:

```
p hat: 0.3999753
Normalized estimation error: 0.00006175000000011033
```

Or, setting the trials to `100_000_000`:

```
p hat: 0.39991376
Normalized estimation error: 0.00021560000000003798
```

Then, setting the trials to `1_000_000_000`:

```
p hat: 0.399994077
Normalized estimation error: 0.000014807500000046936
```
