export interface Baseline {
    version: string,
    description: string,
    status: BaselineStatus,
    createdAt: DateTime,
    createdBy: User,
    deletedAt: DateTime | null,
    deletedBy: User,
    hash: string | null,
}
