import React, {useState} from 'react';
import {timeZones} from "@ultirequiem/timezones";
import Select from "react-select";
import errorIcon from "../assets/error.svg"
import successIcon from "../assets/success.svg"
import refreshIcon from "../assets/refresh.svg"
import {updateCurrentUser, useCurrentUser} from "../utils/api";
import {useMutation} from "@tanstack/react-query";

const timezoneOptions = timeZones.map(tz => ({value: tz, label: tz}))
const browserTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

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

function DashboardPage() {
    const userQuery = useCurrentUser()
    const [status, setStatus] = useState<Status | null>(null)
    const updateMutation = useMutation(updateCurrentUser, {
        onError: () => setStatus(Status.ERROR),
        onSuccess: () => setStatus(Status.SUCCESS),
        onMutate: () => setStatus(Status.WAITING)
    })

    return (
        <div className="flex inline-flex justify-center items-center mt-2 transition-transform">
            {/*TODO: figure out how to use flex layouts instead of this to keep it centered */}
            <div className={status !== null ? "min-w-[2rem]" : ""}/>
            <Select
                defaultMenuIsOpen={userQuery.data?.timezoneId === null}
                defaultInputValue={userQuery.data?.timezoneId === null ? browserTimezone : ""}
                defaultValue={userQuery.data?.timezoneId !== null ? {
                    value: userQuery.data?.timezoneId,
                    label: userQuery.data?.timezoneId
                } : null}
                autoFocus={true}
                isClearable={true}
                options={timezoneOptions}
                onChange={(tz) => updateMutation.mutate({timezone: tz?.value})}
                className="min-w-[15rem] text-black mr-3"
            />
            {status !== null && <StatusIndicator status={status}/>}
        </div>
    );
}

export default DashboardPage;
