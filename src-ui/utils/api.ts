import {useQuery, UseQueryResult} from "@tanstack/react-query";

const API_URL = window.location.origin + "/api"

interface User {
    userId: string,
    timezone: string,
    timezoneId: string,
}

function handleResponse(res: Response): Promise<Response> {
    return res.status === 200
        ? Promise.resolve(res)
        : res.status === 401
            ? Promise.reject("unauthorized")
            : Promise.reject(`${res.statusText} ${res.text()}`)
}

function handleResponseData<T>(res: Response): Promise<T> | null {
    return res.status === 200
        ? res.json() as Promise<T>
        : res.status === 401
            ? null
            : Promise.reject(`${res.statusText} ${res.text()}`)
}

function useCurrentUser(): UseQueryResult<User | null> {
    return useQuery(
        ['user'],
        () => fetch(`${API_URL}/user`).then(handleResponseData),
        {
            refetchOnMount: false,
            refetchOnWindowFocus: false,
            refetchOnReconnect: false,
        }
    )
}

function updateCurrentUser(data: { timezone: string | null | undefined }): Promise<Response> {
    return fetch(
        `${API_URL}/user`,
        {
            body: JSON.stringify(data),
            method: 'PUT',
            headers: {'Content-Type': "application/json"}
        }
    ).then(handleResponse)
}

export {
    useCurrentUser,
    updateCurrentUser,
}
