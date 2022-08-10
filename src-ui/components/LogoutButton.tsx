import React from 'react';
import logoutIcon from '../assets/logout.svg';
import {redirectLogout} from "../utils/api";

function LogoutButton() {
    return <img
        onClick={redirectLogout}
        src={logoutIcon}
        alt="Logout"
        width={35} height={35}
        className="hover:cursor-pointer text-red"
    />
}

export {
    LogoutButton,
}
