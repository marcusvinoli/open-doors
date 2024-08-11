export type DataType = "empty" | "any" | "boolean" | "integer" | "integerPositive" | "integerNegative" | "float" | "option" | "options";

export interface DataKind {};

export interface NullableAny extends DataKind {
    any: "",
}

export interface NullableOption extends DataKind {
    nullableOption: string[],
}

export interface NullableOptions extends DataKind {
    nullableOptions: string[],
}

export interface NullableBoolean extends DataKind {
    nullableBoolean: false,
}

export interface Field {
    attribute: string,
    kind: DataKind,
    key: string,
}

export interface Template {
    fields: Field[],
}
