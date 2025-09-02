export function generateModuleKey(repoPath: string, modulePath: string, version?: string) {
    const dataFormat = `${modulePath.substring(repoPath.length)}-${version ?? 'current'}`;
    const sanitized = dataFormat.toLowerCase().replace(/[^a-z0-9]/g, '');
    const truncated = sanitized.length > 30 ? sanitized.substring(0, 30) : sanitized;
    return truncated;
}
