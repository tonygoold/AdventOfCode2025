These are my solutions for [Advent of Code 2025](https://adventofcode.com/2025/). All solutions were written by me, with occasional reference to known computer science algorithms. Below are my explanations for each day's problems.

# Day 1

This is similar to modular arithmetic and I suspect there is a better solution based on a mathematical interpretation of the operations, where "left N" is treated as "right 100-N". The only optimization I included was to check for rotations greater than or equal to 100, since `N = 100a + b` means it passes zero either `a` or `a+1` times, depending on whether `N = b` passes zero.

# Day 2

I mainly brute forced this. For each N-digit integer, for K = 1 .. N/2, I check if K evenly divides N and, if so, whether the number can be created by repeating the K-digit suffix of that integer N/K times.

When I started on the first half of this problem, I assumed it was going to involve _counting_ how many numbers satisfy the condition and that the ranges in the input would be extremely large, so my initial solution involved breaking down the range into sub-ranges based on even numbers of digits, finding the least and greatest repeated prefixes within the sub-range, and calculating how many repeated numbers occur in the sub-range using just those bounds. For example, in the range `[1234, 5678]`, the prefix `12` is too small since `1212` is not in the range, so the lower bound is `1313`. The prefix `56` isn't too big since `5656` is in the range, so the upper bound is `5656`. The number of repeated numbers within that range is thus `56 - 13 + 1 = 44` (i.e., 1313, 1414, 1515, ..., 5454, 5555, 5656). I thought this was a clever solution to the wrong problem.

# Day 3

This problem can be solved recursively using mathematical induction. Given a string of N digits, to find the largest number that can be formed by selecting K digits in order from the string, we can establish a base case for selecting the first digit by observing:

1. The digit must be selected from the first N - K + 1 digits, otherwise there would not be enough digits to form a K digit number.
2. Since it is the most significant digit, it must be the largest digit in that range. There is no way to form a larger number by starting with a smaller digit.
3. If there are multiple occurrences of the largest digit in that range, we should choose the first (left-most) one. If a solution can be found by starting with the digit at index _i_, then that solution's remaining digits also occur after a digit at index less than _i_.

Therefore a solution starts with the first occurrence of the greatest digit in the range `[0,N-K]`. For the induction step, if the first digit is at index _i_, then a solution is the first digit followed by the greatest K-1 digit number formed from the remaining string in the range [i,N]. Thus the pseudo-code solution is:

```
greatest_k_digit_int(S, K):
  if K == 0:
    return ""
  i := greatest_digit_index(S[0:len(S) - K])
  d := S[i]
  return concat(d, greatest_k_digit_int(S[i:len(S)], K - 1))
```

# Day 4

This is a cellular automaton with only one rule. I solved this by deriving a new grid from the previous grid at every step and counting the number of cells that changed state, stopping when no cells change state. To avoid bounds checking when working around the edges, the grid could be extended on each side with empty cells.

This is similar to doing a convolution on an image, with the additional step of rounding the result to 0 or 1.

# Day 5

Derive a set of disjoint (non-overlapping) ranges equivalent to the input ranges. Given a set _R_ of ranges _r0_ .. _rn_, we want to maintain the invariant, for all _i_, _j_, if x ∈ _ri_ ∪ _rj_, then _i_ = _j_. When we add a range _r_ not already in _R_, there are at most two ranges it overlaps: One containing its lower bound and one containing its upper bound. If either or both exist, we remove them from _R_, merge them with _r_ by taking the minimum lower bound and maximum upper bound, and insert the result back into _R_. For the first part of the problem, an element is included if it is in any range in the set. For the second part of the problem, since the ranges in the set are disjoint, we can sum their lengths.

As a minor optimization, I merged ranges that are overlapping **or adjacent**. This doesn't affect the correctness but it results in a more compact representation, since the ranges `[i,j]` and `[j+1,k]` are disjoint but adjacent, so they can be merged into a single range `[i,k]`.

# Day 6

For the first part, I converted the input into a 2D matrix and applied the operator at the bottom of each column to the operands in that column. This was straightforward, splitting each line by one or more whitespace characters to get each row.

For the second part, I treated the input as a grid of characters and transposed it. This transforms an input like this:

```
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
```

Into this:
```
1  *
24  
356 
    
369+
248 
8   
    
 32*
581 
175 
    
623+
431 
  4 
```

As a result, each line is either empty, an integer, or an integer with an operation at the end. Since both operations are commutative, the lines can be processed in order, starting a new calculation every time an operation is encountered. In my solution, I used `fold` with an array of integers and an operation as the state. If a line ended with an operation, I first updated the current operation and pushed the identity for that operation onto the array. For every line containing a number, I updated the last element of the array with the result of applying the current operation to that element and the number for the current line. Finally, the solution is the sum of the results. Using the above example, the state updates as follows:

```
[], + (initial, the operation is arbitrary since it will immediately be updated)
[1], * (operation found, push identity for *)
[1], * (multiply by 1)
[24], * (multiply by 24)
[8544], * (multiply by 356)
[8544, 0], + (operation found, push identity for +)
[8544, 369], + (add 369)
[8544, 617], + (add 248)
...
[8544, 625, 3253600, 1054], + (add 431)
[8544, 625, 3253600, 1058], + (add 4)
```

# Day 7

Since we only care how many beams are at a given position for each step, we can map positions to beam counts using a map and the solution is straightforward. This is really just a lesson not to track each beam individually, since the number of beams grows exponentially.

# Day 8

This is a [minimum spanning tree](https://en.wikipedia.org/wiki/Minimum_spanning_tree) problem. Since the number of vertices (junction boxes) is small, we can calculate the distances between all vertices to get a set of weighted edges. The first part guides you toward [Kruskal's algorithm](https://en.wikipedia.org/wiki/Kruskal%27s_algorithm), except we don't need the complete set of edges forming the minimum spanning tree, we only need to track the last edge added.
