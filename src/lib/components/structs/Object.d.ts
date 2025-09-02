import type { User } from './User';
import type { Metadata } from './Metadata'
import type { Template } from './Template';
import type { AttributeHashMap } from './Attributes';

export interface Object {
    id: number,
    indexParentId: number,
    indexLevel: string,
    header: string,
    content: string,
    author: User | string, 
    createdAt: Date,
    updatedAt: Date,
    deletedAt: Date | null,
    attributes: AttributeHashMap | null,
    metadata: Metadata | null,
}
