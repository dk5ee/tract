//! # Tensorflow Deploy
//!
//! Tiny, no-nonsense, self contained, portable Tensorflow inference.
//!
//! ## Example
//!
//! ```
//! # extern crate tfdeploy;
//! # extern crate ndarray;
//! # fn main() {
//! use tfdeploy::*;
//!
//! // load a simple model that just add 3 to each input component
//! let model = tfdeploy::for_path("tests/models/plus3.pb").unwrap();
//!
//! // "input" and "output" are tensorflow graph node names.
//! // we build an execution plan for computing output from input
//! let plan = SimplePlan::new(&model, &["input"], &["output"]).unwrap();
//!
//! // run the computation.
//! let input = ndarray::arr1(&[1.0f32, 2.5, 5.0]);
//! let mut outputs = plan.run(vec![input.into()]).unwrap();
//!
//! // take the tensors coming out of the only output node
//! let mut tensors = outputs.pop().unwrap();
//!
//! // grab the first (and only) tensor of this output, and unwrap it as array of f32
//! let tensor = tensors.pop().unwrap().take_f32s().unwrap();
//! assert_eq!(tensor, ndarray::arr1(&[4.0, 5.5, 8.0]).into_dyn());
//! # }
//! ```
//!
//! For a more serious example, see [inception v3 example](https://github.com/kali/tensorflow-deploy-rust/blob/master/examples/inceptionv3.rs).

// TODO: show Plan-based API in doc instead of shortcut

extern crate bit_set;
#[cfg(feature = "blis")]
extern crate blis_src;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate error_chain;
#[cfg(feature = "image_ops")]
extern crate image;
extern crate itertools;
#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[allow(unused_imports)]
#[macro_use]
extern crate ndarray;
extern crate num;
extern crate protobuf;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate objekt;

#[cfg(feature = "serialize")]
extern crate serde;
#[cfg(test)]
extern crate simplelog;
#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate downcast_rs;

#[macro_use]
pub mod analyser;
pub mod dim;
pub mod errors;
pub mod model;
pub mod ops;
pub mod plan;
pub mod streaming;
pub mod tensor;
pub mod tfpb;

pub use errors::*;
use std::path;

pub use model::{Model, Node};
pub use plan::SimplePlan;
pub use tensor::{DatumType, Tensor};

/// Load a Tensorflow protobul model from a file.
pub fn for_path<P: AsRef<path::Path>>(p: P) -> Result<Model> {
    Ok(Model::for_path(p)?)
}

#[cfg(test)]
#[allow(dead_code)]
fn setup_test_logger() {
    use simplelog::{Config, LevelFilter, TermLogger};
    TermLogger::init(LevelFilter::Trace, Config::default()).unwrap()
}
