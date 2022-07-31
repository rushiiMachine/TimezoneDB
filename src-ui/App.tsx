import React from 'react';
import {useCurrentUser} from "./utils/api";
import LoginPage from "./pages/LoginPage";
import './App.scss';

function App() {
    const userQuery = useCurrentUser()

    return (
        <div className="min-h-screen bg-dark-black text-white">
            {userQuery.isError && <h1>huge disaster occurred</h1>}
            {userQuery.isFetched}
        </div>
    );
}

export default App;
