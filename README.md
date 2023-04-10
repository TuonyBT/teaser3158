# teaser3158
Sunday Times Teaser published 2 April 2023
Find 3 distinct non-zero digits a, b, c that meet the following conditions:
N1, N3 and N3 are 3-digit integers made up from permutations of a, b, c such that N1 > N3 > N5 and N1 + N3 + N5 < 2000
N2 = N1 - N3 and N4 = N3 - N5
N2 and N4 are two-digit numbers each made up from two of a, b, c
The first digit of N2 is not the same as the first digit of N1

Sledgehammer approach would take all possible permutations of three from 1..10 and then take all possible combinations of three from this set. 
Trial values of N1, N3 and N5 would then be defined by one such combination, sorted in descending order.

This approach reduces the amount of trial and error thus:
1. Separate a three-digit number into the dot product of: a vector of those three digits in descending order; a vector of powers of ten from 0 to 2 inclusive
2. Work on combinations of three permutations of the powers vector such that any arbitrary vector of three descending digits applied to each combination
  will always produce N1, N3 and N5 in the right order
3. Generate vectors (a, b, c) looping only over values of a, b, c that could produce values of N2 and N4 that are less than 100. This latter requirement
  constrains the possibe values of b relative to a and c relative to b, depending on the specific powers vectors in each sequence
