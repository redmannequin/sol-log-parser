use crate::{
    raw_log::{RawDataLog, RawLog, RawProgramLog},
    Result,
};

use super::{ComputeUnits, Log2};

/// A Raw Structured Log
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawStructuredLog<'a> {
    pub program_id: &'a str,
    pub depth: u8,
    pub result: RawProgramResult<'a>,
    pub program_logs: Vec<RawProgramLog<'a>>,
    pub data_logs: Vec<RawDataLog<'a>>,
    pub return_data: Option<&'a str>,
    pub compute_log: Option<ComputeUnits>,
    pub cpi_logs: Vec<RawStructuredLog<'a>>,
    pub raw_logs: Vec<&'a str>,
}

impl<'a> RawStructuredLog<'a> {
    pub fn from_raw_logs(logs: Vec<RawLog<'a>>) -> Result<Vec<Self>> {
        let log2: Vec<_> = logs.into_iter().map(Log2::from).collect();
        let structured_log = helper_code::RawStructuredLogHelper::from_logs(log2)?;
        Ok(structured_log.into_iter().map(Self::from).collect())
    }
}

/// A Raw Program Result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawProgramResult<'a> {
    Success,
    Err(&'a str),
}

/* *************************************************************************** *
 *  HELPER CODE
 * *************************************************************************** */

mod helper_code {
    use crate::{
        raw_log::{RawDataLog, RawProgramLog},
        structured_log::{ProgramResult, StructuredLog},
    };

    use super::{RawProgramResult, RawStructuredLog};

    impl<'a> From<RawStructuredLogHelper<'a>> for RawStructuredLog<'a> {
        fn from(value: RawStructuredLogHelper<'a>) -> Self {
            Self {
                program_id: value.program_id,
                depth: value.depth,
                result: match value.result {
                    ProgramResult::Success => RawProgramResult::Success,
                    ProgramResult::Err(err) => RawProgramResult::Err(err),
                },
                program_logs: value.program_logs,
                data_logs: value.data_logs,
                return_data: value.return_data,
                compute_log: value.compute_log,
                cpi_logs: value.cpi_logs.into_iter().map(Self::from).collect(),
                raw_logs: value.raw_logs,
            }
        }
    }

    pub type RawStructuredLogHelper<'a> = StructuredLog<
        &'a str,
        ProgramResult<&'a str>,
        RawProgramLog<'a>,
        RawDataLog<'a>,
        &'a str,
        &'a str,
    >;
}
