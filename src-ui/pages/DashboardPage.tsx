import React, {useState} from 'react';
import {timeZones} from "@ultirequiem/timezones";
import Select from "react-select";
import {updateCurrentUser, useCurrentUser} from "../utils/api";
import {useMutation, useQueryClient} from "@tanstack/react-query";
import {Status, StatusIndicator} from "../components/StatusIndicator";

const toSelectOption = (value: string) =>
    ({value: value, label: value})

const timezoneOptions = timeZones.map(toSelectOption)
const browserTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

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
        <div className="inline-flex justify-center items-center mt-2 transition-transform">
            <div className={status !== null ? "min-w-[2rem]" : ""}/>
            {userQuery.data !== undefined && <Select
                defaultMenuIsOpen={!timezoneId}
                defaultInputValue={!timezoneId ? browserTimezone : ""}
                defaultValue={timezoneId ? toSelectOption(timezoneId) : null}
                value={timezoneId ? toSelectOption(timezoneId) : null}
                autoFocus={true}
                isClearable={true}
                options={timezoneOptions}
                onChange={(tz) => updateMutation.mutate({timezone: tz?.value ?? null})}
                className="min-w-[15rem] text-black mr-3"
            />}
            {status !== null && <StatusIndicator status={status}/>}
        </div>
    );
}

export default DashboardPage;
