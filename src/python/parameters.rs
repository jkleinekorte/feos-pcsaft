use crate::parameters::{PcSaftParameters, PcSaftRecord};
use feos_core::joback::JobackRecord;
use feos_core::parameter::{
    BinaryRecord, IdentifierOption, Parameter, ParameterError, PureRecord, SegmentRecord,
};
use feos_core::python::joback::PyJobackRecord;
use feos_core::python::parameter::{PyBinarySegmentRecord, PyChemicalRecord, PyIdentifier};
use feos_core::*;
use numpy::{PyArray2, ToPyArray};
use pyo3::prelude::*;
use std::convert::TryFrom;
use std::rc::Rc;

/// Create a set of PC-Saft parameters from records.
#[pyclass(name = "PcSaftRecord", unsendable)]
#[pyo3(
    text_signature = "(m, sigma, epsilon_k, mu=None, q=None, kappa_ab=None, epsilon_k_ab=None, na=None, nb=None, viscosity=None, diffusion=None, thermal_conductivity=None)"
)]
#[derive(Clone)]
pub struct PyPcSaftRecord(PcSaftRecord);

#[pymethods]
impl PyPcSaftRecord {
    #[new]
    fn new(
        m: f64,
        sigma: f64,
        epsilon_k: f64,
        mu: Option<f64>,
        q: Option<f64>,
        kappa_ab: Option<f64>,
        epsilon_k_ab: Option<f64>,
        na: Option<f64>,
        nb: Option<f64>,
        viscosity: Option<[f64; 4]>,
        diffusion: Option<[f64; 5]>,
        thermal_conductivity: Option<[f64; 4]>,
    ) -> Self {
        Self(PcSaftRecord::new(
            m,
            sigma,
            epsilon_k,
            mu,
            q,
            kappa_ab,
            epsilon_k_ab,
            na,
            nb,
            viscosity,
            diffusion,
            thermal_conductivity,
        ))
    }

    #[getter]
    fn get_m(&self) -> f64 {
        self.0.m
    }

    #[getter]
    fn get_sigma(&self) -> f64 {
        self.0.sigma
    }

    #[getter]
    fn get_epsilon_k(&self) -> f64 {
        self.0.epsilon_k
    }

    #[getter]
    fn get_mu(&self) -> Option<f64> {
        self.0.mu
    }

    #[getter]
    fn get_q(&self) -> Option<f64> {
        self.0.q
    }

    #[getter]
    fn get_kappa_ab(&self) -> Option<f64> {
        self.0.kappa_ab
    }

    #[getter]
    fn get_epsilon_k_ab(&self) -> Option<f64> {
        self.0.epsilon_k_ab
    }

    #[getter]
    fn get_na(&self) -> Option<f64> {
        self.0.na
    }

    #[getter]
    fn get_nb(&self) -> Option<f64> {
        self.0.nb
    }

    #[getter]
    fn get_viscosity(&self) -> Option<[f64; 4]> {
        self.0.viscosity
    }

    #[getter]
    fn get_diffusion(&self) -> Option<[f64; 5]> {
        self.0.diffusion
    }

    #[getter]
    fn get_thermal_conductivity(&self) -> Option<[f64; 4]> {
        self.0.thermal_conductivity
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for PyPcSaftRecord {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

impl_json_handling!(PyPcSaftRecord);

impl_pure_record!(PcSaftRecord, PyPcSaftRecord, JobackRecord, PyJobackRecord);
impl_segment_record!(PcSaftRecord, PyPcSaftRecord, JobackRecord, PyJobackRecord);

/// Create a set of PC-SAFT parameters from records.
///
/// Parameters
/// ----------
/// pure_records : List[PureRecord]
///     pure substance records.
/// binary_records : List[BinaryRecord], optional
///     binary saft parameter records
/// substances : List[str], optional
///     The substances to use. Filters substances from `pure_records` according to
///     `search_option`.
///     When not provided, all entries of `pure_records` are used.
/// search_option : {'Name', 'Cas', 'Inchi', 'IupacName', 'Formula', 'Smiles'}, optional, defaults to 'Name'.
///     Identifier that is used to search substance.
///
/// Returns
/// -------
/// PcSaftParameters
#[pyclass(name = "PcSaftParameters", unsendable)]
#[pyo3(
    text_signature = "(pure_records, binary_records=None, substances=None, search_option='Name')"
)]
#[derive(Clone)]
pub struct PyPcSaftParameters(pub Rc<PcSaftParameters>);

impl_parameter!(PcSaftParameters, PyPcSaftParameters);
impl_parameter_from_segments!(PcSaftParameters, PyPcSaftParameters);

#[pymethods]
impl PyPcSaftParameters {
    #[getter]
    fn get_pure_records(&self) -> Vec<PyPureRecord> {
        self.0
            .pure_records
            .iter()
            .map(|r| PyPureRecord(r.clone()))
            .collect()
    }

    #[getter]
    fn get_k_ij<'py>(&self, py: Python<'py>) -> &'py PyArray2<f64> {
        self.0.k_ij.view().to_pyarray(py)
    }

    fn _repr_markdown_(&self) -> String {
        self.0.to_markdown()
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for PyPcSaftParameters {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}
