import type { BaselineStatus } from "./BaselineStatus";

export interface Baseline {
    version: string,
    description: string,
    status: BaselineStatus,
    createdAt: DateTime,
    createdBy: User  | string,
    deletedAt: DateTime | null,
    deletedBy: User | string | null,
    hash: string | null,
}
