import {useQuery, UseQueryResult} from "@tanstack/react-query";

const API_URL = window.location.origin + "/api"

interface User {
    userId: string,
    timezone: string,
    timezoneId: string,
}

function handleResponse<T>(res: Response) {
    return res.status === 200
        ? res.json() as Promise<User>
        : res.status === 401
            ? null
            : Promise.reject(`${res.statusText} ${res.text()}`)
}

function useCurrentUser(): UseQueryResult<User | null> {
    return useQuery(
        ['user'],
        () => fetch(`${API_URL}/user`).then(handleResponse),
        {
            refetchOnMount: false,
            refetchOnWindowFocus: false,
            refetchOnReconnect: false,
        }
    )
}

export {
    useCurrentUser,
}
