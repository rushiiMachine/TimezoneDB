import React from 'react';
import logoutIcon from '../assets/logout.svg';

const logoutUser = () => {
    window.location.replace("/api/auth/logout");
};

function LogoutButton() {
    return <img
        onClick={logoutUser}
        src={logoutIcon}
        alt="Logout"
        width={35} height={35}
        className="hover:cursor-pointer text-red"
    />
}

export {
    LogoutButton,
}
