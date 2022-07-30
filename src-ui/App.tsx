import React from 'react';
import {useQuery} from "@tanstack/react-query";
import {getUser} from "./utils/api";
import LoginPage from "./pages/LoginPage";

function App() {
    const userQuery = useQuery(['user'], getUser)
    console.log(userQuery)

    return (
        <div className="min-h-screen bg-dark-black text-white">
            {userQuery.isError && <h1>huge disaster occurred</h1>}
            {userQuery.isFetched}
        </div>
    );
}

export default App;
