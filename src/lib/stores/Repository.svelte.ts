import type { Repository } from "$lib/components/structs/Repo";

let _repository : Repository | null = $state(null)

export function repository() {
    return _repository;
}

export function setRepository(repository: Repository | null) {
    _repository = repository;
}
