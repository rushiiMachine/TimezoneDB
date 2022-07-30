const API_URL = window.location.origin + "/api"

interface User {
    userId: string,
    timezone: string,
    timezoneId: string,
}

function getUser() {
    return fetch(`${API_URL}/user`)
        .then(res => res.json())
        .then(json => json as User)
}

export {
    getUser,
}
