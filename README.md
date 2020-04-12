# Introduction

## Incremental Computation

Incremental Computation is a software feature in which a program tries
to save time by only recomputing the data that has changed when the
input changes or a structure has been modified. A program *P* can be
said to be incremental if repeating program *P* is faster than
from-scratch re-computation. A common pattern for incrementalizing a
program is called memoization or tabling. Memoization often means
storing a table of input and the results of some expensive calculation
on those inputs for future lookup, preventing recomputation of the
expensive calculation. A simplified example of this would be to think
about Arithmetic. Let us say there is a standard algorithm designed to
calculate the result of arithmetic expression if it was asked to
calculate $4+6\div2$ ; this algorithm — assuming it follows order of
operations – would calculate $6\div2$  then $4+3$  and return 7 as
expected. However, if then next, it was asked to calculate $5+6\div2$ ,
the algorithm would calculate $6\div2$  then $5+3$  redoing the
division operation. While in this case, division is an effortless
operation, imagine this was being done on a computer where division was
very complex or instead of division it was some very complex function
then storing the result of $6\div2$  in a memo table would drastically
improve the speed of the calculation. However, memoization does have
some problem for it to be an effective technique; the computation it is
being applied to must be a pure function. That is to say, the only
things that can affect the output are the inputs. Basically, there must
not be any external state that can affect the output. For a contrived
example of why let’s go back to the arithmetic example, however, this
time, if the time is after 12:15 pm instead of returning the computed
arithmetic expression, our function returns 0. Now, if we had $6\div2$ 
stored in our memo table as 3, then if our function were run after 12:15
pm our function would return the wrong result. Incremental Computation
can also be about managing evolving data structures such as an example
from the paper "Purely Functional Incremental Computing: where they show
an example of an incremental program in which the goal is only to have a
view of odd values. They show an example how if you have already
computed the view of odd values if you need to add a number to the view
rather than adding all the new numbers to the source regardless of
whether it is even or odd then recomputing the view you can take the
already computed view and insert the new odd values into it

## Nested Incremental Computation

Nested Incremental Computation is basically what it sounds like, just as
nested loops are loops within loops nested incremental programs is one
incremental program within another incremental program. While in
concept, this seems simple, often incremental programs are large and
domain-specific, so finding an example that can incrementally produce
data for another program to consume can be a slight challenge.

# Methods

The nested incremental program I created was a program that calculates
arithmetic expressions and keeps a set of those expressions in sorted
order. To program this, I chose to use the Rust Programming language,
for its incredible type system and its functional affordances. By using
Rust, this would ensure that when I ran the benchmarks, that I knew
could take a while the chances of a runtime error was incredibly low, by
eliminating theses class of bugs it allowed me to run more complex
benchmarks with peace of mind.

## Arithmetic Expressions

The arithmetic expressions are stored in trees representing prefix
notation expressions. Each tree is represented by these `Rust` structs.

```rust
#[derive(Debug, Clone)]
pub struct Node {
    pub operation: Operation,
    pub value: Option<i128>,
    pub children: Vec<Node>,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Val,
}
```

Each node consists of an Operation, an optional value, and a Vec of
nodes which hold the children of the operation, a special case of a node
is a leaf in which operation is set to `Val`, value is always set and
the children Vec is empty. To calculate the value of a tree, the `calc`
function is called on the root of that tree this function checks if
`value` is set, if it then just returns that otherwise, the `calc`
function is recursively called on both children and performs the defined
operation on the result of the two calculations, after which the value
calculated is stored in value. Because the function is recursively
called on every node after the first call, every subtree will have its
value set, and this is what allows for incremental computation if the
tree is modified.

To incrementally calculate the new value of a tree when a change is made
the function recursively parses down to the defined spot of the change,
then it makes the changes it then stores the previous value set in a
temporary variable, after that it the `calc` function is called on the
subtree where that node of modification is the root, once it gets the
newly calculated value if the value is the same as the previous value
stored in temporary value the function returns false and this bubbles
back up the tree, however, if the value changes the function returns
true. This process is repeated for the parent node until the root of the
whole tree is reached.

## Set of Trees

The goal of this section was to store a set of trees in sorted order and
be able to incrementally add a new element, or modify an element of the
set. Just as the example of showing odd values in a list from the
introduction section showed, that rather than starting from scratch with
the whole list of values adding the new ones then recalculating the view
of just odd values, an incremental and faster way of doing that would be
to take the already calculated view and just add the new values into
that view, I thought was rather than when inserting a value or modifying
the set starting from an unsorted list, why not save the work that has
already been done and just find the location that the new element should
go and insert it there saving the sort.\* Based on this, my initial idea
was to use a vector. However, when adding an element, this would require
moving every element down by one after the insertion point. This
behaviour was undesirable. So for this stage, a B-Tree was used as the
collection to store the trees. A B-tree is a specialized form of search
tree which, just like a Red-Black Tree, is balanced. However, a B-Tree
optimizes for disk space by allowing each node to contain more than one
object and have more than two children to utilize the benefits of
continuous blocks of data while storing things on disk on in memory. By
utilizing a balanced tree structure, this dealt with order problems as
the nature of the collection is that it’s sorted. As well now when a
value is inserted it has a better time complexity of
$\mathcal{O}(\log{}n)$  compared to the vector which would have a
complexity of $\mathcal{O}(\log{}n + n)$  so $\mathcal{O}(n)$  as the
search would be $\mathcal{O}(\log{}n)$  if a binary search were used
but then since you have to move all the elements one to the right that
has a complexity of $\mathcal{O}(n)$ . As well the better incremental
complexity doesn’t trade-off for a worse from scratch complexity. Let us
say the complexity of evaluating the arithmetic expression is
$\mathcal{O}(m)$  since the complexity of an insert is
$\mathcal{O}(\log{}n)$  as already stated, due to that the complexity
of a from-scratch calculation would be
$\mathcal{O}(m \times n \times \log{}n)$  which is the same as with the
vector from the scratch calculation.

# Experiments

To test the speedups of the incremental programs I wrote, I utilized the
Criterion.rs Library. This library is a robust and statistics-driven
benchmarking framework for the Rust programming language. By utilizing
this framework, it let me easily test the speed in CPU Cycles both from
scratch and incremental calculations for the Non-nested and Nested
version of my programs. As well by utilizing Critrion’s `iter_bached`
function this allowed each iteration of the benchmark to have set which
was crucial for the from scratch benchmarks as reusing the structures
would no longer make it a from scratch calculation.

## Non-Nested

The process used to test the speed-ups incremental arithmetic expression
evaluation is as follows. First, a balanced tree with a depth on $n$ 
was generated ($\mathcal{T}(n)$ ). Next, this tree is cloned, and the
`calc` function is called on it and stored ($\mathcal{T^\prime}(n)$ ).
To test the speed ups three functions were benchmarked, this first being
a baseline of a from-scratch calculation of $\mathcal{T}(n)$ , the
second being a defined modification and calculation of
$\mathcal{T}(n)$ , the final being the same defined modification and
calculation but on $\mathcal{T^\prime}(n)$ this being the benchmark for
the incremental calculation. These three functions were run with a $n$ 
from 5 to 20. The Criterion Framework collects 100 samples with each
sample while doing as many iterations as can fit in a set specified
time. The time chosen being 3000s as all functions regardless of n could
run enough iterations of > 5050, which is the set iterations required
for 100 samples in the given time.

<figure>
    <img src="lines-32.png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 1. Results of Non-Nested Benchmarks[^1]</em></figcaption>
</figure>

The result above illustrate that as expected the the incremental
computation is around the same speed for smaller tree and as the trees
got larger the overheard of storing the values for the incremental
computation got outweigh by the significant amount of calculations
needed for very large trees. As well it was pretty unsurprising that the
from scratch baseline and the from scratch recalculation were almost
identical.

## Nested

The process to test the speed-ups for the a set of Trees first a set
with size k of a trees with a depth m was generated $\mathcal{S} =$ 
$\{\mathcal{T}(n)_1, \mathcal{T}(n)_2, ... \mathcal{T}(n)_k\}$ . Next
this set was cloned and sorted and stored $\mathcal{S}^\prime =$ 
$\{\mathcal{T^\prime}(n)_1, \mathcal{T^\prime}(n)_2, ... \mathcal{T^\prime}(n)_k\}$ to
test the speedups two functions were tested.[^2] The First function is
a from scratch sort this mean none of the tree were calculated and the
BTreeSet being used to as the collection was empty. The second function
was a incremental version this was done by using $\mathcal{S}^\prime$ 
and by taking one element modifying it, incrementally recalculating it
and reinserting it. This was planed to be done on a $n$  from 10 to 20
each for $k$ s of 100, 600, 1200, and 2000 however the time requirement
for each benchmark so big that by the time the benchmark for to n=20
k=100 the benchmark was taking 43 hours to run the minimum number of
iterations(5050) required for the benchmark so unfortunately all the
benchmarks were not able to finish. So the data collected will only be
for a K of 100

<figure>
    <img src="image001.png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 2. Result of Nested Benchmarks[^3]</em></figcaption>
</figure>

These result just showed the drastic improvement incremental calculation
can have on the speed of a program. The was unsurprising though because
the sort from scratch can be thought of just n number of "incremental"
calculations as to sort you just insert and to resort with a new value
you just insert nothing about the semantics change just the amount in
which it happens.

# Conclusion

Obviously the results mostly speak for them self, however there are
obviously some caveats in saying that, first of all in general my code
is written to optimize for incremental computation. For example in from
scratch version of the nested calculation maybe it would be faster you
have stored them in a vector and used something like quick-sort rather
than storing in the B-Tree, as it would need to get used later. As well
obviously my example is a little contrived as math for computers is
fairly trivial and adding and entire tree structure on the arithmetic
expressions would slow down the calculations. As well when you look at
the amount of operations required to have the incremental version be
worth it, it does bring in to questions the realistic use case for
something that acts similar, the incremental calculations started to
really pull away at a tree depth of 14 that means the trees has 32767
nodes, if this optimizes for the large cases and that a large cases that
would never happen is it worth it to do. However as a proof of concept
the ability to utilize incremental computation in a nested way shows
that if you can create something and have it be a strong use case for
incremental computation it may be something to look in to.

# Appendix

<figure>
    <img src="violin(5).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 3. Violin Plot of non-nested benchmark with n of 5</em></figcaption>
</figure>
<figure>
    <img src="violin(6).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 4. Violin Plot of non-nested benchmark with n of 6</em></figcaption>
</figure>
<figure>
    <img src="violin(7).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 5. Violin Plot of non-nested benchmark with n of 7</em></figcaption>
</figure>
<figure>
    <img src="violin(8).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 6. Violin Plot of non-nested benchmark with n of 8</em></figcaption>
</figure>
<figure>
    <img src="violin(9).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 7. Violin Plot of non-nested benchmark with n of 9</em></figcaption>
</figure>
<figure>
    <img src="violin(10).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 8. Violin Plot of non-nested benchmark with n of 10</em></figcaption>
</figure>
<figure>
    <img src="violin(11).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 9. Violin Plot of non-nested benchmark with n of 11</em></figcaption>
</figure>
<figure>
    <img src="violin(12).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 10. Violin Plot of non-nested benchmark with n of 12</em></figcaption>
</figure>
<figure>
    <img src="violin(13).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 11. Violin Plot of non-nested benchmark with n of 13</em></figcaption>
</figure>
<figure>
    <img src="violin(14).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 12. Violin Plot of non-nested benchmark with n of 14</em></figcaption>
</figure>
<figure>
    <img src="violin(15).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 13. Violin Plot of non-nested benchmark with n of 15</em></figcaption>
</figure>
<figure>
    <img src="violin(16).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 14. Violin Plot of non-nested benchmark with n of 16</em></figcaption>
</figure>
<figure>
    <img src="violin(17).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 15. Violin Plot of non-nested benchmark with n of 17</em></figcaption>
</figure>
<figure>
    <img src="violin(18).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 16. Violin Plot of non-nested benchmark with n of 18</em></figcaption>
</figure>
<figure>
    <img src="violin(19).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 17. Violin Plot of non-nested benchmark with n of 19</em></figcaption>
</figure>
<figure>
    <img src="violin(20).png" style="margin: 0 auto; max-width: 400px;">
    <figcaption><em>Fig 18. Violin Plot of non-nested benchmark with n of 20</em></figcaption>
</figure>

[^1]: As the number get large quick comparisons at smaller input values look skewed for more detailed comparison check index

[^2]: This time only two were used as from the previous result the from scratch and a modified but still from scratch should produce a similar result so for time reason only a from scratch was tested.

[^3]: Once again the smaller values of n look like they are the same for the from scratch and nested calculations this is just a illusion created by the higher n values having such large cycle counts
