V3(
    InkProject {
        registry: PortableRegistry {
            types: [
                PortableType {
                    id: 0,
                    ty: Type {
                        path: Path {
                            segments: [
                                "MyCustomStruct",
                            ],
                        },
                        type_params: [],
                        type_def: Composite(
                            TypeDefComposite {
                                fields: [
                                    Field {
                                        name: Some(
                                            "0",
                                        ),
                                        ty: UntrackedSymbol {
                                            id: 1,
                                            marker: PhantomData,
                                        },
                                        type_name: Some(
                                            "custom_u32",
                                        ),
                                        docs: [],
                                    },
                                    Field {
                                        name: Some(
                                            "1",
                                        ),
                                        ty: UntrackedSymbol {
                                            id: 1,
                                            marker: PhantomData,
                                        },
                                        type_name: Some(
                                            "custom_u32",
                                        ),
                                        docs: [],
                                    },
                                    Field {
                                        name: Some(
                                            "2",
                                        ),
                                        ty: UntrackedSymbol {
                                            id: 1,
                                            marker: PhantomData,
                                        },
                                        type_name: Some(
                                            "custom_u32",
                                        ),
                                        docs: [],
                                    },
                                ],
                            },
                        ),
                        docs: [],
                    },
                },
                PortableType {
                    id: 1,
                    ty: Type {
                        path: Path {
                            segments: [],
                        },
                        type_params: [],
                        type_def: Primitive(
                            U32,
                        ),
                        docs: [],
                    },
                },
                PortableType {
                    id: 2,
                    ty: Type {
                        path: Path {
                            segments: [],
                        },
                        type_params: [],
                        type_def: Primitive(
                            I32,
                        ),
                        docs: [],
                    },
                },
            ],
        },
        layout: Enum(
            EnumLayout {
                dispatch_key: LayoutKey {
                    key: [
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                },
                variants: {
                    Discriminant(
                        0,
                    ): StructLayout {
                        fields: [],
                    },
                    Discriminant(
                        1,
                    ): StructLayout {
                        fields: [],
                    },
                    Discriminant(
                        2,
                    ): StructLayout {
                        fields: [],
                    },
                },
            },
        ),
        spec: ContractSpec {
            constructors: [
                ConstructorSpec {
                    label: "default",
                    selector: Selector(
                        [
                            2,
                            34,
                            255,
                            24,
                        ],
                    ),
                    payable: false,
                    args: [
                        MessageParamSpec {
                            label: "foo",
                            ty: TypeSpec {
                                ty: UntrackedSymbol {
                                    id: 0,
                                    marker: PhantomData,
                                },
                                display_name: Path {
                                    segments: [],
                                },
                            },
                        },
                    ],
                    docs: [],
                },
            ],
            messages: [
                MessageSpec {
                    label: "get",
                    selector: Selector(
                        [
                            37,
                            68,
                            74,
                            254,
                        ],
                    ),
                    mutates: false,
                    payable: false,
                    args: [],
                    return_type: ReturnTypeSpec {
                        opt_type: Some(
                            TypeSpec {
                                ty: UntrackedSymbol {
                                    id: 2,
                                    marker: PhantomData,
                                },
                                display_name: Path {
                                    segments: [
                                        "i32",
                                    ],
                                },
                            },
                        ),
                    },
                    docs: [],
                },
            ],
            events: [],
            docs: [],
        },
    },
)
