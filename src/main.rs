use std::fs::read_to_string;

use contract_transcode::ContractMessageTranscoder;
use ink_metadata::{
    layout::{EnumLayout, Layout, StructLayout},
    ConstructorSpec, ContractSpec, InkProject, MessageSpec, MetadataVersioned, ReturnTypeSpec,
    TypeSpec,
};
use ink_primitives::{Key, KeyPtr};

#[allow(unused)]
fn read() {
    let args: Vec<String> = std::env::args().collect();
    let json = read_to_string(&args[1]).unwrap();
    let metadata: MetadataVersioned = serde_json::from_str(&json).unwrap();
    println!("{:#?}", &metadata);
    if let MetadataVersioned::V3(ink_project) = &metadata {
        println!("{:#?}", ink_project.registry());
    }
    println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
}

fn clike_enum(key_ptr: &mut KeyPtr) -> Layout {
    EnumLayout::new(
        key_ptr.advance_by(1),
        vec![
            (0.into(), StructLayout::new(vec![])),
            (1.into(), StructLayout::new(vec![])),
            (2.into(), StructLayout::new(vec![])),
        ],
    )
    .into()
}

fn dummy_contract() -> ContractSpec {
    ContractSpec::new()
        .constructors(vec![ConstructorSpec::from_label("default")
            .selector([2u8, 34u8, 255u8, 24u8])
            .payable(Default::default())
            .args(Vec::new())
            .docs(Vec::new())
            .done()])
        .messages(vec![MessageSpec::from_label("get")
            .selector([37u8, 68u8, 74u8, 254u8])
            .mutates(false)
            .payable(false)
            .args(Vec::new())
            .docs(Vec::new())
            .returns(ReturnTypeSpec::new(TypeSpec::with_name_segs::<i32, _>(
                vec!["i32"].into_iter().map(AsRef::as_ref),
            )))
            .done()])
        .done()
}

fn main() {
    let layout = clike_enum(&mut KeyPtr::from(Key::from([0x00; 32])));
    let spec = dummy_contract();

    let ink_project = InkProject::new(layout, spec);
    let transcoder = ContractMessageTranscoder::new(&ink_project);
    //println!("{:#?}", MetadataVersioned::from(ink_project));

    let constructor = "default";
    let args: [&str; 0] = [];
    let data = transcoder.encode(constructor, &args).unwrap();

    println!("Encoded constructor data {:?}", data);
}
