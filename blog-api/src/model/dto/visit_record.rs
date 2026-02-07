use crate::entity::visit_record;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitRecord {
    pub pv: i32,
    pub uv: i32,
    pub date: String,
}

impl From<visit_record::Model> for VisitRecord {
    fn from(value: visit_record::Model) -> Self {
        Self {
            pv: value.pv,
            uv: value.uv,
            date: value.date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitRecordChart {
    pub date: Vec<String>,
    pub pv: Vec<i32>,
    pub uv: Vec<i32>,
}

impl VisitRecordChart {
    pub fn from_records(records: Vec<VisitRecord>) -> Self {
        let mut dates = Vec::with_capacity(records.len());
        let mut pv = Vec::with_capacity(records.len());
        let mut uv = Vec::with_capacity(records.len());

        for record in records {
            dates.push(record.date);
            pv.push(record.pv);
            uv.push(record.uv);
        }

        Self {
            date: dates,
            pv,
            uv,
        }
    }
}
