import numpy as np
import puan_rspy as pr

theory = pr.TheoryPy([
    pr.StatementPy(0, (0,1), pr.AtLeastPy([1,2],-1)),
    pr.StatementPy(1, (0,1), pr.AtLeastPy([3,4], 1)),
    pr.StatementPy(2, (0,1), pr.AtLeastPy([5,6],-2)),
    pr.StatementPy(3, (0,1), None),
    pr.StatementPy(4, (0,1), None),
    pr.StatementPy(5, (0,1), None),
    pr.StatementPy(6, (0,1), None),
])

ph = theory.to_polyhedron(True, False)
for (lineq, _b) in zip(np.array_split(ph.a.val, ph.a.nrows), ph.b):
    print(lineq, _b)

for solution in theory.solve([{3: 1}, {4: 1}, {5: 1}, {6: 1}, {3:1, 4:1}], True):
    print(solution.x, solution.z, solution.status_code)