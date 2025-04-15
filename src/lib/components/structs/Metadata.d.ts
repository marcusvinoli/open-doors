import type { Link } from './Link';
import type { ObjectStatus } from './ObjectStatus';

export type Metadata = {
    status: ObjectStatus,
    inboundLinks: Vec<Link> | null,
    outboundLinks: Vec<Link> | null,
}
