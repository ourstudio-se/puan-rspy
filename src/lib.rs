use puanrs::*;
use pyo3::prelude::*;

#[pyclass]
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

    pub fn to_lineqs(&self) -> Vec<GeLineqPy> {
        let t = Theory {
            id: String::from(""),
            statements: self.statements.iter().map(|stat| {
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

        return t.to_lineqs().iter().map(|lineq| {
            GeLineqPy {
                bias: lineq.bias,
                bounds: lineq.bounds.to_vec(),
                coeffs: lineq.coeffs.to_vec(),
                indices: lineq.indices.to_vec()
            }
        }).collect()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn puan_rspy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TheoryPy>()?;
    m.add_class::<StatementPy>()?;
    m.add_class::<AtLeastPy>()?;
    m.add_class::<GeLineqPy>()?;
    Ok(())
}