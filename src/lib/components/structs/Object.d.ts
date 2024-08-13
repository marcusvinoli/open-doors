import type { Author } from "./Author";
import type { User } from "./User";

export interface IHash {
    [key: string]: string;
}

export interface Link {
    path: string,
    module: string,
    object: number,
}

export interface Links {
    inbound: Link[],
    outbound: Link[],
}

export interface Object {
    id: number,
    level: string,
    header: string,
    content: string,
    author: User | string, 
    isActive: boolean,
    isNormative: boolean,
    isRequirement: boolean,
    createdAt: Date,
    updatedAt: Date,
    deletedAt: Date | null,
    customFields: IHash | null,
    outboundLinks: Link[] | null,
}

export interface ObjectView {
    object: Object,
    inboundLinks: Link[] = [],
    isDraft: boolean = false,
    hasChanges: boolean = false,
}
