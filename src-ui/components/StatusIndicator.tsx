import React from "react";
import successIcon from "../assets/success.svg";
import refreshIcon from "../assets/refresh.svg";
import errorIcon from "../assets/error.svg";

enum Status {
    WAITING,
    SUCCESS,
    ERROR,
}

function StatusIndicator({status}: { status: Status }) {
    return <img
        width={30}
        alt={status === Status.SUCCESS ? "Success" : status === Status.WAITING ? "Waiting" : "Error"}
        src={status === Status.SUCCESS ? successIcon : status === Status.WAITING ? refreshIcon : errorIcon}
        className={status === Status.WAITING ? "spinning" : ""}
    />
}

export {
    Status,
    StatusIndicator,
}