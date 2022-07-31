import React from 'react';
import DiscordLegacyIcon from "../assets/discord_legacy_icon.svg"

function LoginPage() {
    const redirectLogin = () => {
        window.location.replace("/api/auth")
    }

    return (
        <div className="flex flex-col justify-center items-center">
            <button onClick={redirectLogin}
                    className="border-blurple border-[16px] rounded bg-blurple hover:bg-blurple-lighter hover:border-blurple-lighter max-w-fit inline-flex transition-all hover:p-2 hover:rounded-md">
                <img src={DiscordLegacyIcon} alt="Discord logo" width={25} className="mr-3 mt-1"/>
                <p className="text-2xl">Login with Discord</p>
            </button>
        </div>
    );
}

export default LoginPage;
