use crate::pseudopotential::upf::UPFData;

pub mod pseudopotential;

fn main() {
    let input = r#"
    <UPF version="2.0.1">
        <PP_HEADER element="Si" zp="4">
            <INFO>Some nested data</INFO>
        </PP_HEADER>
        <PP_R>
            0.0 1.1 2.2
        </PP_R>
    </UPF>
    "#;

    let upf_data = UPFData::parse(input).unwrap();
    println!("Parsed UPF data: {:?}", upf_data);
}