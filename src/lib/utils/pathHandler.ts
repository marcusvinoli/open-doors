import * as path from 'path';

export function relativePath(fullPath: string): string {
    const normalizedPath = path.normalize(fullPath);

    const dirPath = path.dirname(normalizedPath);

    const folderName = path.basename(dirPath);

    return folderName;
}

export function absolutePath(basePath: string, relativePath: string): string {
    return path.resolve(basePath, relativePath);
}
