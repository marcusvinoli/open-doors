export type AttributeKind = 'general' | 'string' | 'integer' | 'real' | 'date' | 'time' | 'dateTime' | 'boolean' | 'enumeration' | 'optional' | 'user' | any;

export interface Attribute {
    isMandatory: bool;
    kind: AttributeKind;
    name: string;
    description: string;
    key: string;
}
