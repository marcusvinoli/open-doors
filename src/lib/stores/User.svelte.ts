import { User } from "$lib/components/structs/User"

let _user : User | null = $state(null);

export function user() {
    return _user;
}

export function setUser(user: User | null) {
    _user = user;
}
