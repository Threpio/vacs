import {create} from "zustand/react";

export type CallListItem = { type: "IN" | "OUT"; time: string; name: string; number: string; };

type CallListState = {
    callList: CallListItem[];
    actions: {
        addCall: (call: CallListItem) => void;
        clearCallList: () => void;
    };
};

export const useCallListStore = create<CallListState>()((set, get) => ({
    callList: [
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
        {type: "IN", time: "10:00", name: "John Doe", number: "+49 123 456 7890"},
    ],
    actions: {
        addCall: (call: CallListItem) => {
            set({callList: [call, ...get().callList]});
        },
        clearCallList: () => {
            set({callList: []});
        },
    }
}));