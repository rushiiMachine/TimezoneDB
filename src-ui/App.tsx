import React from 'react';
import {useIsLoggedIn} from "./utils/api";
import LoginPage from "./pages/LoginPage";
import DashboardPage from "./pages/DashboardPage";
import icon from './assets/icon.svg';
import errorIcon from "./assets/error.svg"
import refreshIcon from "./assets/refresh.svg"
import './App.scss';

function App() {
    const loggedInQuery = useIsLoggedIn()

    return (
        <div className="app bg-not-black text-white">
            <div className="min-h-screen flex flex-col justify-center">
                <div className="inline-flex max-w-fit self-center mb-4">
                    <img src={icon} width={80} alt="TimezoneDB icon"/>
                    <a className="text-6xl font-bold mt-2 ml-2 hover:underline"
                       href="https://github.com/DiamondMiner88/TimezoneDB">TimezoneDB</a>
                </div>

                {loggedInQuery.isFetching &&
                    <img alt="loading indicator" src={refreshIcon} width={40} className="spinning self-center"/>}

                {loggedInQuery.isSuccess &&
                    (loggedInQuery.data ? <DashboardPage/> : <LoginPage/>)}

                {!!loggedInQuery.error && <div className="inline-flex max-w-fit self-center">
                    <img alt="error icon" src={errorIcon} width={35}/>
                    <p className="text-2xl ml-2">{String(loggedInQuery.error)}</p>
                </div>}
            </div>
        </div>
    );
}

export default App;
