import type { User } from './User';
import type { Metadata } from './Metadata'

export interface IHash {
    [key: string]: string;
}

export interface Object {
    id: number,
    parentLevel: number,
    indexLevel: number,
    header: string,
    content: string,
    author: User | string, 
    createdAt: Date,
    updatedAt: Date,
    deletedAt: Date | null,
    attributes: IHash | null,
    metadata: Metadata,
}
