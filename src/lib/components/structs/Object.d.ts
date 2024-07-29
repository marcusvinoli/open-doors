import type { Author } from "./Author";

export interface Link {
    module: string,
    object: string,
}

export interface Links {
    input: Link[],
    output: Link[],
}

export interface CustomFields extends IHash {}

export interface Object {
    id: bigint,
    number: string,
    header: string,
    content: string,
    author: Author,
    isActive: boolean,
    isNormative: boolean,
    createdAt: Date,
    updatedAt: Date,
    deletedAt: Date | null,
    customFiels: CustomFields | null,
    links: Links,
    isDraft: boolean,
}
