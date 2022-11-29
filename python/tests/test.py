import puan_rspy as pr

def test_simple_theory_construction_and_solving():
    theory = pr.TheoryPy([
        pr.StatementPy(0, (0,1), pr.AtLeastPy([1,2],-1, pr.SignPy.Positive)),
        pr.StatementPy(1, (0,1), pr.AtLeastPy([3,4], 1, pr.SignPy.Negative)),
        pr.StatementPy(2, (0,1), pr.AtLeastPy([5,6],-2, pr.SignPy.Positive)),
        pr.StatementPy(3, (0,1), None),
        pr.StatementPy(4, (0,1), None),
        pr.StatementPy(5, (0,1), None),
        pr.StatementPy(6, (0,1), None),
    ])

    ph = theory.to_ge_polyhedron(True, False)
    for solution, objective_value, status_code in theory.solve([{3: 1}, {4: 1}, {5: 1}, {6: 1}, {3:1, 4:1}], True):
        assert status_code != 5
