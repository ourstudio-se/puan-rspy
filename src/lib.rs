use std::collections::HashMap;

use puanrs::*;
use puanrs::polyopt::*;
use pyo3::prelude::*;

#[pyclass]
pub struct MatrixPy {
    val: Vec<f64>,
    nrows: usize,
    ncols: usize
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

#[pyclass]
pub struct IntegerSolutionPy {
    pub x: Vec<i64>,
    pub z: i64,
    pub status_code: usize
}

#[derive(Debug)]
#[pyclass]
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

#[derive(Debug)]
#[pyclass]
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

#[pyclass]
pub struct PolyhedronPy {
    /// The left-hand side of linear constraints on the form $ a + b + c \ge x $.
    pub a: MatrixPy,
    /// The right-hand side of linear constraints as described above.
    pub b: Vec<f64>,
    /// Upper and lower bounds (`lower_bound`, `upper_bound`) of the variables given by `a`.
    pub variables: Vec<VariableFloatPy>
}


#[pyclass]
#[derive(Clone)]
pub struct GeLineqPy {
    pub bias: i64,
    pub bounds: Vec<(i64,i64)>,
    pub coeffs: Vec<i64>,
    pub indices: Vec<u32>
}

#[pymethods]
impl GeLineqPy {

    #[new]
    pub fn new(bias: i64, bounds: Vec<(i64,i64)>, coeffs: Vec<i64>, indices: Vec<u32>) -> GeLineqPy {
        return GeLineqPy { bias: bias, bounds: bounds, coeffs: coeffs, indices: indices };
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
                bias: self.bias, 
                bounds: self.bounds.to_vec(),
                coeffs: self.coeffs.to_vec(),
                indices: self.indices.to_vec()
            },
            &GeLineq {
                bias: other.bias, 
                bounds: other.bounds,
                coeffs: other.coeffs,
                indices: other.indices
            },
        );
        return match result {
            Some(glin) => Ok(Some(GeLineqPy {bias: glin.bias, bounds: glin.bounds, coeffs: glin.coeffs, indices: glin.indices})),
            None => Ok(None)
        }
    }

    pub fn merge_conj(&self, other: GeLineqPy) -> PyResult<Option<GeLineqPy>> {
        let result: Option<GeLineq> = GeLineq::merge_conj(
            &GeLineq {
                bias: self.bias, 
                bounds: self.bounds.to_vec(),
                coeffs: self.coeffs.to_vec(),
                indices: self.indices.to_vec()
            },
            &GeLineq {
                bias: other.bias, 
                bounds: other.bounds,
                coeffs: other.coeffs,
                indices: other.indices
            },
        );
        return match result {
            Some(glin) => Ok(Some(GeLineqPy {bias: glin.bias, bounds: glin.bounds, coeffs: glin.coeffs, indices: glin.indices})),
            None => Ok(None)
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AtLeastPy {
    ids: Vec<u32>,
    bias: i64
}

#[pymethods]
impl AtLeastPy {
    
    #[new]
    pub fn new(ids: Vec<u32>, bias: i64) -> AtLeastPy {
        return AtLeastPy { ids: ids, bias: bias }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct StatementPy {
    pub variable: Variable,
    pub expression: Option<AtLeastPy>
}

#[pymethods]
impl StatementPy {
    #[new]
    pub fn new(id: u32, bounds: (i64,i64), expression: Option<AtLeastPy>) -> StatementPy {
        return StatementPy {
            variable: Variable { id: id, bounds: bounds },
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
                            ids: a.ids.to_vec()
                        }
                    ),
                    None => None
                },
                variable: stat.variable
            }
        }).collect()
    };
}

#[pyclass]
pub struct TheoryPy {
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
                bias: lineq.bias,
                bounds: lineq.bounds.to_vec(),
                coeffs: lineq.coeffs.to_vec(),
                indices: lineq.indices.to_vec()
            }
        }).collect()
    }

    pub fn to_polyhedron(&self, active: bool, reduced: bool) -> PolyhedronPy {
        let intern_polyhedron = _to_theory_helper(&self).to_polyhedron(active, reduced);
        return PolyhedronPy { 
            a: MatrixPy {
                val: intern_polyhedron.a.val,
                ncols: intern_polyhedron.a.ncols,
                nrows: intern_polyhedron.a.nrows,
            }, 
            b: intern_polyhedron.b, 
            variables: intern_polyhedron.variables.iter().map(|v| VariableFloatPy {id: v.id, bounds: v.bounds}).collect()
        }
    }

    pub fn solve(&self, objectives: Vec<HashMap<u32, f64>>) -> Vec<IntegerSolutionPy> {
        return _to_theory_helper(&self).solve(objectives).into_iter().map(|sol| {
            IntegerSolutionPy {
                status_code: sol.status_code,
                x: sol.x,
                z: sol.z,
            }
        }).collect();
    }
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
    Ok(())
}