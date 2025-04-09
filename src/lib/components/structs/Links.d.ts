import type Link from 'Link';

export type Links = {
    inboundLinks: Record<number, Link[]>;
    outboundLinks: Record<number, Link[]>;
}
