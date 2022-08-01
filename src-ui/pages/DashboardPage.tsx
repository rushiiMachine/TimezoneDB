import React, {useState} from 'react';
import {timeZones} from "@ultirequiem/timezones";
import Select from "react-select";
import errorIcon from "../assets/error.svg"
import successIcon from "../assets/success.svg"
import refreshIcon from "../assets/refresh.svg"
import {updateCurrentUser, useCurrentUser} from "../utils/api";
import {useMutation, useQueryClient} from "@tanstack/react-query";

const toSelectOption = (value: string) =>
    ({value: value, label: value})

const timezoneOptions = timeZones.map(toSelectOption)
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
    const queryClient = useQueryClient()
    const userQuery = useCurrentUser()
    const timezoneId = userQuery.data?.timezoneId;

    const [status, setStatus] = useState<Status | null>(null)
    const updateMutation = useMutation(updateCurrentUser, {
        onMutate: () => setStatus(Status.WAITING),
        onError: () => setStatus(Status.ERROR),
        onSuccess: async () => {
            setStatus(Status.SUCCESS)
            await queryClient.invalidateQueries(['user'])
        },
    })

    return (
        <div className="flex inline-flex justify-center items-center mt-2 transition-transform">
            <div className={status !== null ? "min-w-[2rem]" : ""}/>
            {userQuery.data !== undefined && <Select
                defaultMenuIsOpen={!timezoneId}
                defaultInputValue={!timezoneId ? browserTimezone : ""}
                defaultValue={timezoneId ? toSelectOption(timezoneId) : null}
                value={timezoneId ? toSelectOption(timezoneId) : null}
                autoFocus={!timezoneId}
                isClearable={true}
                options={timezoneOptions}
                onChange={(tz) => updateMutation.mutate({timezone: tz?.value})}
                className="min-w-[15rem] text-black mr-3"
            />}
            {status !== null && <StatusIndicator status={status}/>}
        </div>
    );
}

export default DashboardPage;
