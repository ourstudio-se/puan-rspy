<div style="text-align:center;">
<h1>puan-rspy</h1>
<i>Puan's python and rust connection package.</i>
</div>

## What is it?
Puan is a collection of tools for discrete optimization modelling in various programming languages. Puan-rspy binds [Puan's Rust package](https://github.com/ourstudio-se/puan-rust) into a Python package. Check the link for more information about the tools inside.

## Install
Using `pip` package manager just
```bash
pip install puan-rspy
```

## Usage
Here we construct a problem using the propositional logic classes within puan-rspy.
We construct a `TheoryPy` using a list of statements. Each statement has a variable and (optional) an `AtLeast`-proposition.

As an example, we'll create a model with only boolean variables `"A","B","C","a","b","x","y"`. Since a `StatementPy` only takes an integer as id parameter, we imagine them all being 0, 1, 2, ..., 6, respectively. We want to say that if any of `"a" or "b"` is selected, then `"x" and "y"` must be selected. We do this by creating new statements connecting a new variable to these variables. First we create `C = x + y -2` (which is the ```pr.StatementPy(2, (0,1), pr.AtLeastPy([5,6],-2, pr.SignPy.Positive))```). Then we remeber from logic class that an implication statement (`B -> C`) is equivilant to `!B or C`. So for the `B = "a" or "b"` statement, we negate it first into `not "a" and not "b"` which then described as `B = -a - b + 0`. Last, we connec B and C into a final statement `A = B + C`.

```python
import puan_rspy as pr

model = pr.TheoryPy([
    # This first statement has id=0, bounds=(0,1) and an AtLeast proposition
    # saying that the sum of value of variable "1" and variable "2" and -1 must
    # be greater or equal than zero. In other words, at least one of variable "1"
    # and variable "2" must be 1.
    pr.StatementPy(0, (0,1), pr.AtLeastPy([1,2],-1, pr.SignPy.Positive)),
    pr.StatementPy(1, (0,1), pr.AtLeastPy([3,4], 1, pr.SignPy.Negative)),
    pr.StatementPy(2, (0,1), pr.AtLeastPy([5,6],-2, pr.SignPy.Positive)),

    # These are independent variable declarations, e.i. variables that are
    # not dependent on other variables
    pr.StatementPy(3, (0,1), None),
    pr.StatementPy(4, (0,1), None),
    pr.StatementPy(5, (0,1), None),
    pr.StatementPy(6, (0,1), None),
])

# Here we use puan-rust's internal solver for solving our model with these
# five different objectives, and then print the result.
for solution, objective_value, status_code in theory.solve([{3: 1}, {4: 1}, {5: 1}, {6: 1}, {3:1, 4:1}], True):
    print(solution, objective_value, status_code)

# Which prints:
# {3: 1, 6: 0, 5: 0, 4: 0} 1 5
# {3: 0, 4: 1, 5: 0, 6: 0} 1 5
# {5: 1, 4: 0, 3: 0, 6: 0} 1 5
# {5: 0, 6: 1, 4: 0, 3: 0} 1 5
# {4: 1, 5: 1, 3: 1, 6: 1} 2 5 
```
