export class User {
    name: string;
    email: string;

    constructor(name: string, email?: string) {
        this.name = name;
        this.email = email ?? "";
    }

    toString(): string {
        if (this.email.trim() === "") {
            return this.getFirstAndLastName();
        }
        return `${this.getFirstAndLastName()} <${this.email}>`;
    }

    static fromString(serialized: string): User {
        const match = serialized.match(/^(.*) <(.*)>$/);
        if (!match) {
            throw new Error(`Invalid format for User: ${serialized}`);
        }
        const [_, name, email] = match;
        return new User(name, email);
    }

    getFirstAndLastName(): string {
        const nameParts = this.name.trim().split(/\s+/);
        
        if (nameParts.length === 0) {
            return '';
        }
        
        const firstName = nameParts[0];
        const lastName = nameParts[nameParts.length - 1];
        return `${firstName} ${lastName}`;
    }

}
