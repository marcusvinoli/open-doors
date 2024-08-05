import type { Author } from "./Author";
import type { User } from "./User";

export interface Link {
    module: string,
    object: string,
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
}

export interface ObjectView {
    object: Object,
    links: Link[] = [],
    isDraft: boolean = false,
    hasChanges: boolean = false,
}
