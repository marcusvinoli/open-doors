export class User {
    name: string;
    email: string;

    constructor(name: string, email: string) {
        this.name = name;
        this.email = email;
    }

    toString(): string {
        return `${this.name} <${this.email}>`;
    }

    static fromString(serialized: string): User {
        const match = serialized.match(/^(.*) <(.*)>$/);
        if (!match) {
            throw new Error(`Invalid format for User: ${serialized}`);
        }
        const [_, name, email] = match;
        return new User(name, email);
    }
}
