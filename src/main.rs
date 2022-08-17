use std::fs::read_to_string;

use ink_metadata::{
    layout::{EnumLayout, Layout, StructLayout},
    ConstructorSpec, ContractSpec, InkProject, MessageParamSpec, MessageSpec, MetadataVersioned,
    ReturnTypeSpec, TypeSpec,
};
use ink_primitives::{Key, KeyPtr};
use scale_info::{
    build::{Fields, FieldsBuilder, NamedFields, TypeBuilder},
    form::MetaForm,
    MetaType, Path, Type, TypeInfo,
};

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

fn runtime_meta_type() -> Type<MetaForm> {
    let mut fields: FieldsBuilder<NamedFields> = Fields::named();
    let custom_u32_id = 1;
    let custom_u32_type_info = <u32 as TypeInfo>::type_info;
    for i in 0..3 {
        fields.field_mut(|f| {
            f.ty_meta(MetaType::new_custom(custom_u32_id, custom_u32_type_info))
                .name(i.to_string())
                .type_name("custom_u32".to_string())
        })
    }
    let ty = TypeBuilder::default()
        .path(Path::<MetaForm>::from_segments(vec!["MyCustomStruct".to_string()]).unwrap())
        .composite(fields);
    ty
}

fn dummy_contract() -> ContractSpec {
    let meta_type = MetaType::new_custom(123, runtime_meta_type);
    let type_spec = <TypeSpec<MetaForm>>::new_from_ty(meta_type);

    ContractSpec::new()
        .constructors(vec![ConstructorSpec::from_label("default".to_string())
            .selector([2u8, 34u8, 255u8, 24u8])
            .payable(Default::default())
            .args(vec![MessageParamSpec::new("foo".to_string())
                .of_type(type_spec)
                .done()])
            .docs(Vec::new())
            .done()])
        .messages(vec![MessageSpec::from_label("get".to_string())
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
    println!("{:#?}", MetadataVersioned::from(ink_project));
}
