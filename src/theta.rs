mod rdt;
pub mod rthu;
pub mod rdta;
pub mod rdtb;
pub mod rdtc;
pub mod rdtg;
pub mod rdtl;
pub mod rdl2;

use serde::Serialize;

#[derive(Debug)]
pub struct RawBox {
    pub data: Vec<u8>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct SerializableThetaMeta {
    pub rdta: Option<rdta::RdtaBox>,
    pub rdtb: Option<rdtb::RdtbBox>,
    pub rdtc: Option<rdtc::RdtcBox>,
    pub rdtg: Option<rdtg::RdtgBox>,
    pub rdtl: Option<rdtl::RdtlBox>,
    pub rdl2: Option<rdl2::Rdl2Box>,
    #[serde(rename = "@mod")]
    pub _mod: String,
    #[serde(rename = "@swr")]
    pub _swr: String,
    #[serde(rename = "@day")]
    pub _day: String,
    #[serde(rename = "@xyz")]
    pub _xyz: String,
    #[serde(rename = "@mak")]
    pub _mak: String,
    #[serde(rename = "manu")]
    pub manu: String,
    #[serde(rename = "modl")]
    pub modl: String,
}
#[derive(Debug)]
pub struct ThetaMeta {
    pub rthu: Option<rthu::RthuBox>,
    pub rmkn: Option<RawBox>,
    pub rdt1_8: Option<RawBox>,
    pub rdt9: Option<RawBox>,
    pub rdta: Option<rdta::RdtaBox>,
    pub rdtb: Option<rdtb::RdtbBox>,
    pub rdtc: Option<rdtc::RdtcBox>,
    pub rdtd: Option<RawBox>,
    pub rdtg: Option<rdtg::RdtgBox>,
    pub rdth: Option<RawBox>,
    pub rdti: Option<RawBox>,
    pub rdtl: Option<rdtl::RdtlBox>,
    pub rdl2: Option<rdl2::Rdl2Box>,
    pub _mod: String,
    pub _swr: String,
    pub _day: String,
    pub _xyz: String,
    pub _mak: String,
    pub manu: String,
    pub modl: String,
}

impl ThetaMeta {
    pub fn to_serializable(&self) -> SerializableThetaMeta {
        SerializableThetaMeta {
            rdta: self.rdta.clone(),
            rdtb: self.rdtb.clone(),
            rdtc: self.rdtc.clone(),
            rdtg: self.rdtg.clone(),
            rdtl: self.rdtl.clone(),
            rdl2: self.rdl2.clone(),
            _mod: self._mod.clone(),
            _swr: self._swr.clone(),
            _day: self._day.clone(),
            _xyz: self._xyz.clone(),
            _mak: self._mak.clone(),
            manu: self.manu.clone(),
            modl: self.modl.clone(),
        }
    }
}
