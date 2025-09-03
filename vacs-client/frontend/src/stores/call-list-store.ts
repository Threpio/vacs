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
        {type: "IN", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "IN", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "OUT", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "IN", time: "14:49", name: "LOVV_CTR", number: "999999"},
        {type: "IN", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "OUT", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "OUT", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "IN", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "OUT", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "IN", time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {type: "IN", time: "14:49", name: "LOVV_CTR", number: "4123123"},
    ],
    actions: {
        addCall: (call: CallListItem) => {
            set({callList: [...get().callList, call]});
        },
        clearCallList: () => {
            set({callList: []});
        },
    }
}));