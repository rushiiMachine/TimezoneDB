import React from 'react';
import {useCurrentUser} from "./utils/api";
import LoginPage from "./pages/LoginPage";
import DashboardPage from "./pages/DashboardPage";
import icon from './assets/icon.svg';
import './App.scss';

function App() {
    const userQuery = useCurrentUser()

    return (
        <div className="app bg-not-black text-white">
            <div className="min-h-screen flex flex-col justify-center">
                <div className="inline-flex max-w-fit self-center mb-4">
                    <img src={icon} width={80} alt="TimezoneDB icon"/>
                    <p className="text-6xl font-bold mt-1 ml-2">TimezoneDB</p>
                </div>
                {userQuery.data?.userId ? <DashboardPage/> : <LoginPage/>}
            </div>
            {/*<div className="min-h-[25vh] pl-32 pr-32">*/}
            {/*    <hr className="border-t-2 border-t-dark-black"/>*/}
            {/*    <div className="inline-flex ml-40 mt-10 ">*/}
            {/*footer*/}
            {/*    </div>*/}
            {/*</div>*/}
        </div>
    );
}

export default App;
