use std::{fs::File, io::BufReader, path::Path};

pub mod theta;
use theta::{rdl2, rdta, rdtb, rdtc, rdtg, rdtl, rthu, RawBox, ThetaMeta};

static ALWAYS_INCLUDED_BOXES: &[&str] = &["@mod", "@swr", "@day", "@xyz", "@mak", "manu", "modl"];

pub fn parse<P: AsRef<Path>>(
    filename: &P,
    target_boxes: Option<&[String]>,
) -> Option<(mp4::Mp4Reader<BufReader<File>>, Option<ThetaMeta>)> {
    let f = File::open(filename).ok()?;
    let size = f.metadata().ok()?.len();
    let reader = BufReader::new(f);
    let mp4 = mp4::Mp4Reader::read_header(reader, size).ok()?;
    let moov = &mp4.moov;
    let udta = &moov.udta;

    let mut theta_meta = ThetaMeta {
        rthu: None,
        rmkn: None,
        rdt1_8: None,
        rdt9: None,
        rdta: None,
        rdtb: None,
        rdtc: None,
        rdtd: None,
        rdtg: None,
        rdth: None,
        rdti: None,
        rdtl: None,
        rdl2: None,
        _mod: String::new(),
        _swr: String::new(),
        _day: String::new(),
        _xyz: String::new(),
        _mak: String::new(),
        manu: String::new(),
        modl: String::new(),
    };
    if let Some(udta_box) = udta {
        for child in &udta_box.children {
            if ALWAYS_INCLUDED_BOXES.contains(&child.name.as_str()) {
                match_box(&child.name, &child.data, &mut theta_meta);
            } else if let Some(targets) = target_boxes {
                if targets.contains(&"all".to_string()) || targets.contains(&child.name) {
                    match_box(&child.name, &child.data, &mut theta_meta);
                }
            }
        }
    }

    if theta_meta.modl.contains("RICOH THETA") {
        Some((mp4, Some(theta_meta)))
    } else {
        Some((mp4, None))
    }
}

fn match_box(name: &String, data: &Vec<u8>, theta_meta: &mut ThetaMeta) {
    match name.as_str() {
        "RTHU" => theta_meta.rthu = Some(rthu::RthuBox { data: data.clone() }),
        "RMKN" => theta_meta.rmkn = Some(RawBox { data: data.clone() }),
        "RDT1-8" => theta_meta.rdt1_8 = Some(RawBox { data: data.clone() }),
        "RDT9" => theta_meta.rdt9 = Some(RawBox { data: data.clone() }),
        "RDTA" => theta_meta.rdta = Some(rdta::RdtaBox::read(&data)),
        "RDTB" => theta_meta.rdtb = Some(rdtb::RdtbBox::read(&data)),
        "RDTC" => theta_meta.rdtc = Some(rdtc::RdtcBox::read(&data)),
        "RDTD" => theta_meta.rdtd = Some(RawBox { data: data.clone() }),
        "RDTG" => theta_meta.rdtg = Some(rdtg::RdtgBox::read(&data)),
        "RDTH" => theta_meta.rdth = Some(RawBox { data: data.clone() }),
        "RDTI" => theta_meta.rdti = Some(RawBox { data: data.clone() }),
        "RDTL" => theta_meta.rdtl = Some(rdtl::RdtlBox::read(&data)),
        "RDL2" => theta_meta.rdl2 = Some(rdl2::Rdl2Box::read(&data)),
        "@mod" => theta_meta._mod = String::from_utf8_lossy(&data).to_string(),
        "@swr" => theta_meta._swr = String::from_utf8_lossy(&data).to_string(),
        "@day" => theta_meta._day = String::from_utf8_lossy(&data).to_string(),
        "@xyz" => theta_meta._xyz = String::from_utf8_lossy(&data).to_string(),
        "@mak" => theta_meta._mak = String::from_utf8_lossy(&data).to_string(),
        "manu" => theta_meta.manu = String::from_utf8_lossy(&data).to_string(),
        "modl" => theta_meta.modl = String::from_utf8_lossy(&data).to_string(),
        _ => {}
    }
}
