export type AttributeKind = 'general' | 'string' | 'integer' | 'real' | 'date' | 'time' | 'dateTime' | 'boolean' | { singleOption: string[] } | { multipleOptions: string[] } | 'user' ;


export interface Attribute {
    isMandatory: bool;
    kind: AttributeKind;
    name: string;
    description: string;
    key: string;
}

export interface AttributeHashMap {
    [key: string] : string,
}
