export interface ModuleManifest {
    name: string,
    prefix: string,
    separator: string,
    description: string,
}

export interface ModuleTemplate {

}

export type LinkType = "output" | "input";

export type Link = {
    to: string,
    from: string,
    type: LinkType,
}

export type ObjectType = "requirement" | "text" | "title" | "header";

export interface Object {
    id: bigint,
    index: string,
    header: string,
    text: string,
    type: ObjectType,
    attributes: IHash = {},
    links: Link[],
}

export interface Collection {

}