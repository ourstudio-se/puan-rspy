use std::collections::HashMap;

use puanrs::*;
use puanrs::linalg::*;
use puanrs::polyopt::*;
use pyo3::prelude::*;

#[pyclass(module="Linprog")]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct CsrMatrixPy {
    /// The value for an index `i` in `row` is representing a row index in a virtual matrix
    pub row: Vec<i64>,
    /// The value for an index `j` in `col` is representing a column index in a virtual matrix
    pub col: Vec<i64>,
    /// The value for an element is the value for the cell (`i`,`j`) in a virtual matrix
    pub val: Vec<f64>
}

#[pymethods]
impl CsrMatrixPy {
    
    #[new]
    pub fn new(row: Vec<i64>, col: Vec<i64>, val: Vec<f64>) -> CsrMatrixPy {
        CsrMatrixPy { row, col, val }
    }

    #[getter]
    pub fn val(&self) -> PyResult<Vec<f64>> {
        return Ok(self.val.to_vec())
    }

    #[getter]
    pub fn row(&self) -> PyResult<Vec<i64>> {
        return Ok(self.row.clone())
    }

    #[getter]
    pub fn col(&self) -> PyResult<Vec<i64>> {
        return Ok(self.col.clone())
    }

    pub fn to_matrix(&self) -> MatrixPy {
        let matrix: Matrix = CsrMatrix {
            col: self.col.clone(),
            row: self.row.clone(),
            val: self.val.clone(),
        }.to_matrix();

        return MatrixPy {
            ncols: matrix.ncols,
            nrows: matrix.nrows,
            val: matrix.val,
        }
    }

}

#[pyclass(module="Linprog")]
pub struct MatrixPy {
    pub val: Vec<f64>,
    pub nrows: usize,
    pub ncols: usize
}

impl Clone for MatrixPy {
    fn clone(&self) -> Self {
        return MatrixPy {
            val : self.val.to_vec(),
            ncols: self.ncols,
            nrows: self.nrows
        }
    }
}

#[pymethods]
impl MatrixPy {

    #[new]
    pub fn new(val: Vec<f64>, nrows: usize, ncols: usize) -> MatrixPy {
        MatrixPy { val, nrows, ncols }
    }

    #[getter]
    pub fn val(&self) -> PyResult<Vec<f64>> {
        return Ok(self.val.to_vec())
    }

    #[getter]
    pub fn nrows(&self) -> PyResult<usize> {
        return Ok(self.nrows)
    }

    #[getter]
    pub fn ncols(&self) -> PyResult<usize> {
        return Ok(self.ncols)
    }
}

#[pyclass(module="Linprog")]
pub struct IntegerSolutionPy {
    pub x: Vec<i64>,
    pub z: i64,
    pub status_code: usize
}

#[pymethods]
impl IntegerSolutionPy {

    #[new]
    pub fn new(status_code: usize, x: Vec<i64>, z: i64) -> IntegerSolutionPy {
        IntegerSolutionPy { x, status_code, z }
    }

    #[getter]
    pub fn x(&self) -> PyResult<Vec<i64>> {
        return Ok(self.x.to_vec())
    }

    #[getter]
    pub fn z(&self) -> PyResult<i64> {
        return Ok(self.z)
    }

    #[getter]
    pub fn status_code(&self) -> PyResult<usize> {
        return Ok(self.status_code)
    }
}

#[derive(Debug)]
#[pyclass(module="Polyopt")]
pub struct VariableFloatPy {
    pub id      : u32,
    pub bounds  : (f64, f64)
}

impl Clone for VariableFloatPy {
    fn clone(&self) -> Self {
        return VariableFloatPy { 
            id: self.id, 
            bounds: self.bounds 
        }
    }
}

#[pymethods]
impl VariableFloatPy {

    #[new]
    pub fn new(id: u32, bounds: (f64, f64)) -> VariableFloatPy {
        VariableFloatPy { id, bounds }
    }

    #[getter]
    pub fn id(&self) -> PyResult<u32> {
        return Ok(self.id)
    }

    #[getter]
    pub fn bounds(&self) -> PyResult<(f64,f64)> {
        return Ok(self.bounds)
    }
}

#[derive(Debug)]
#[pyclass(module="Polyopt")]
pub struct VariablePy {
    pub id      : u32,
    pub bounds  : (i64, i64)
}

impl Clone for VariablePy {
    fn clone(&self) -> Self {
        return VariablePy { 
            id: self.id, 
            bounds: self.bounds 
        }
    }
}

#[pymethods]
impl VariablePy {

    #[new]
    pub fn new(id: u32, bounds: (i64, i64)) -> VariablePy {
        VariablePy { id, bounds }
    }
}

#[pyclass(module="Linprog")]
pub struct CsrPolyhedronPy {
    /// The left-hand side of linear constraints on the form $ a + b + c \ge x $ as a compressed sparse matrix.
    pub a: CsrMatrixPy,
    /// The right-hand side of linear constraints as described above.
    pub b: Vec<f64>,
    /// Upper and lower bounds (`lower_bound`, `upper_bound`) of the variables given by `a`.
    pub variables: Vec<VariableFloatPy>,
    // Index of rows in `a`.
    pub index: Vec<Option<u32>>
}

#[pymethods]
impl CsrPolyhedronPy {

    #[new]
    pub fn new(a: CsrMatrixPy, b: Vec<f64>, variables: Vec<VariableFloatPy>, index: Vec<Option<u32>>) -> CsrPolyhedronPy {
        CsrPolyhedronPy { a, b, variables, index }
    }

    #[getter]
    pub fn a(&self) -> PyResult<CsrMatrixPy> {
        return Ok(self.a.clone())
    } 

    #[getter]
    pub fn b(&self) -> PyResult<Vec<f64>> {
        return Ok(self.b.to_vec())
    } 

    #[getter]
    pub fn variables(&self) -> PyResult<Vec<VariableFloatPy>> {
        return Ok(self.variables.to_vec())
    } 

    #[getter]
    pub fn index(&self) -> PyResult<Vec<Option<u32>>> {
        return Ok(self.index.to_vec())
    }

    pub fn to_dense_polyhedron(&self) -> PolyhedronPy {
        return PolyhedronPy {
            a: self.a.to_matrix(),
            b: self.b.clone(),
            index: self.index.clone(),
            variables: self.variables.clone()
        }
    }
}


#[pyclass(module="Linprog")]
pub struct PolyhedronPy {
    /// The left-hand side of linear constraints on the form $ a + b + c \ge x $.
    pub a: MatrixPy,
    /// The right-hand side of linear constraints as described above.
    pub b: Vec<f64>,
    /// Variables given by `a`.
    pub variables: Vec<VariableFloatPy>,
    /// Id on each row
    pub index: Vec<Option<u32>>
}

#[pymethods]
impl PolyhedronPy {

    #[new]
    pub fn new(a: MatrixPy, b: Vec<f64>, variables: Vec<VariableFloatPy>, index: Vec<Option<u32>>) -> PolyhedronPy {
        PolyhedronPy { a, b, variables, index }
    }

    #[getter]
    pub fn a(&self) -> PyResult<MatrixPy> {
        return Ok(self.a.clone())
    } 

    #[getter]
    pub fn b(&self) -> PyResult<Vec<f64>> {
        return Ok(self.b.to_vec())
    } 

    #[getter]
    pub fn variables(&self) -> PyResult<Vec<VariableFloatPy>> {
        return Ok(self.variables.to_vec())
    } 

    #[getter]
    pub fn index(&self) -> PyResult<Vec<Option<u32>>> {
        return Ok(self.index.to_vec())
    } 

    pub fn solve(&self, objectives: Vec<HashMap<u32, f64>>) -> Vec<IntegerSolutionPy> {
        let polyhedron = Polyhedron {
            a: Matrix {
                val: self.a.val.clone(),
                ncols: self.a.ncols,
                nrows: self.a.nrows,
            },
            b: self.b.clone(),
            index: self.index.clone(),
            variables: self.variables.iter().map(|variable| VariableFloat {id: variable.id, bounds: variable.bounds }).collect(),
        };
        let _objectives: Vec<Vec<f64>> = objectives.iter().map(|x| {
            let mut vector = vec![0.0; self.variables.len()];
            for (k, v) in x.iter() {
                let pot_index = polyhedron.variables.iter().position(|y| y.id == (*k));
                if let Some(index) = pot_index {
                    vector[index] = *v;
                }
            }
            return vector;
        }).collect();
        return _objectives.iter().map(|objective| {
            let ilp = solver::IntegerLinearProgram {
                ge_ph: polyhedron.to_owned(),
                eq_ph: Default::default(),
                of: objective.to_vec(),
            };
            let solution = ilp.solve();
            return IntegerSolutionPy { x: solution.x, z: solution.z, status_code: solution.status_code }
        }).collect();
    }
}


#[pyclass(module="Linprog")]
#[derive(Clone)]
pub struct GeLineqPy {
    pub id: Option<u32>,
    pub bias: i64,
    pub bounds: Vec<(i64,i64)>,
    pub coeffs: Vec<i64>,
    pub indices: Vec<u32>
}

#[pymethods]
impl GeLineqPy {

    #[new]
    pub fn new(id: Option<u32>, bias: i64, bounds: Vec<(i64,i64)>, coeffs: Vec<i64>, indices: Vec<u32>) -> GeLineqPy {
        return GeLineqPy { id: id, bias: bias, bounds: bounds, coeffs: coeffs, indices: indices };
    }

    #[getter]
    pub fn id(&self) -> PyResult<Option<u32>> {
        return Ok(self.id)
    } 

    #[getter]
    pub fn bias(&self) -> PyResult<i64> {
        return Ok(self.bias)
    } 

    #[getter]
    pub fn bounds(&self) -> PyResult<Vec<(i64,i64)>> {
        return Ok(self.bounds.to_vec())
    } 

    #[getter]
    pub fn coeffs(&self) -> PyResult<Vec<i64>> {
        return Ok(self.coeffs.to_vec())
    } 

    #[getter]
    pub fn indices(&self) -> PyResult<Vec<u32>> {
        return Ok(self.indices.to_vec())
    } 

    pub fn merge_disj(&self, other: GeLineqPy)  -> PyResult<Option<GeLineqPy>> {
        let result: Option<GeLineq> = GeLineq::merge_disj(
            &GeLineq {
                id: self.id,
                bias: self.bias, 
                bounds: self.bounds.to_owned(),
                coeffs: self.coeffs.to_owned(),
                indices: self.indices.to_owned(),
            },
            &GeLineq {
                id: other.id,
                bias: other.bias, 
                bounds: other.bounds,
                coeffs: other.coeffs,
                indices: other.indices
            },
        );
        return match result {
            Some(glin) => Ok(Some(GeLineqPy {id: glin.id, bias: glin.bias, bounds: glin.bounds, coeffs: glin.coeffs, indices: glin.indices})),
            None => Ok(None)
        }
    }

    pub fn merge_conj(&self, other: GeLineqPy) -> PyResult<Option<GeLineqPy>> {
        let result: Option<GeLineq> = GeLineq::merge_conj(
            &GeLineq {
                id: self.id,
                bias: self.bias, 
                bounds: self.bounds.to_owned(),
                coeffs: self.coeffs.to_owned(),
                indices: self.indices.to_owned()
            },
            &GeLineq {
                id: other.id,
                bias: other.bias, 
                bounds: other.bounds,
                coeffs: other.coeffs,
                indices: other.indices
            },
        );
        return match result {
            Some(glin) => Ok(Some(GeLineqPy {id: glin.id, bias: glin.bias, bounds: glin.bounds, coeffs: glin.coeffs, indices: glin.indices})),
            None => Ok(None)
        }
    }
}
#[pyclass(module="Theory")]
#[derive(Clone)]
pub enum SignPy {
    Positive,
    Negative 
}

impl SignPy{
    fn convert(&self) -> Sign{
        return match self {
            SignPy::Positive => Sign::Positive,
            SignPy::Negative => Sign::Negative
        }
    }
}

#[pyclass(module="Theory")]
#[derive(Clone)]
pub struct AtLeastPy {
    #[pyo3(get, set)]
    ids: Vec<u32>,
    #[pyo3(get, set)]
    bias: i64,
    #[pyo3(get, set)]
    sign: SignPy
}

#[pymethods]
impl AtLeastPy {
    
    #[new]
    pub fn new(ids: Vec<u32>, bias: i64, sign: SignPy) -> AtLeastPy {
        return AtLeastPy { ids: ids, bias: bias, sign: sign }
    }
}

#[pyclass(module="Theory")]
#[derive(Clone)]
pub struct StatementPy {
    #[pyo3(get, set)]
    pub variable: VariableFloatPy,
    #[pyo3(get, set)]
    pub expression: Option<AtLeastPy>
}

#[pymethods]
impl StatementPy {
    #[new]
    pub fn new(id: u32, bounds: (f64, f64), expression: Option<AtLeastPy>) -> StatementPy {
        return StatementPy {
            variable: VariableFloatPy { id: id, bounds: bounds },
            expression: expression
        }
    }
}

fn _to_theory_helper(theory_py: &TheoryPy) -> Theory {
    return Theory {
        id: String::from(""),
        statements: theory_py.statements.iter().map(|stat| {
            Statement {
                expression: match &stat.expression {
                    Some(a) => Some(
                        AtLeast {
                            bias: a.bias,
                            ids: a.ids.to_vec(),
                            sign: a.sign.convert()
                        }
                    ),
                    None => None
                },
                variable: Variable { 
                    id: stat.variable.id, 
                    bounds: (
                        stat.variable.bounds.0 as i64, 
                        stat.variable.bounds.1 as i64
                    ) 
                }
            }
        }).collect()
    };
}

#[pyclass(module="Theory")]
pub struct TheoryPy {
    #[pyo3(get, set)]
    pub statements: Vec<StatementPy>
}

#[pymethods]
impl TheoryPy {

    #[new]
    pub fn new(statements: Vec<StatementPy>) -> TheoryPy {
        return TheoryPy { statements: statements }
    }

    pub fn to_lineqs(&self, active: bool, reduced: bool) -> Vec<GeLineqPy> {
        return _to_theory_helper(&self).to_lineqs(active, reduced).iter().map(|lineq| {
            GeLineqPy {
                id: lineq.id,
                bias: lineq.bias,
                bounds: lineq.bounds.to_owned(),
                coeffs: lineq.coeffs.to_owned(),
                indices: lineq.indices.to_owned(),
            }
        }).collect()
    }

    pub fn to_ge_polyhedron(&self, active: bool, reduced: bool) -> CsrPolyhedronPy {
        let intern_polyhedron = _to_theory_helper(&self).to_ge_polyhedron(active, reduced);
        return CsrPolyhedronPy { 
            a: CsrMatrixPy {
                val: intern_polyhedron.a.val,
                col: intern_polyhedron.a.col,
                row: intern_polyhedron.a.row,
            }, 
            b: intern_polyhedron.b, 
            variables: intern_polyhedron.variables.iter().map(|v| VariableFloatPy {id: v.id, bounds: v.bounds}).collect(),
            index: intern_polyhedron.index
        }
    }

    pub fn solve(&self, objectives: Vec<HashMap<u32, f64>>, reduce_polyhedron: bool) -> Vec<(HashMap<u32, i64>, i64, usize)> {
        return _to_theory_helper(&self).solve(objectives, reduce_polyhedron);
    }
}

#[pyfunction]
pub fn py_optimized_bit_allocation_64(v: Vec<i64>) -> PyResult<Vec<i64>>{
    Ok(linalg::optimized_bit_allocation_64(&v))
}

/// A Python module implemented in Rust.
#[pymodule]
fn puan_rspy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<VariableFloatPy>()?;
    m.add_class::<MatrixPy>()?;
    m.add_class::<PolyhedronPy>()?;
    m.add_class::<TheoryPy>()?;
    m.add_class::<VariablePy>()?;
    m.add_class::<StatementPy>()?;
    m.add_class::<AtLeastPy>()?;
    m.add_class::<GeLineqPy>()?;
    m.add_class::<SignPy>()?;
    m.add_function(wrap_pyfunction!(py_optimized_bit_allocation_64, m)?)?;
    Ok(())
}
