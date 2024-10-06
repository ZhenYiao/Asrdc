import {createJSONStorage, devtools, persist} from 'zustand/middleware'
import { immer } from 'zustand/middleware/immer'
import { create } from "zustand"


interface ZustandStarterState {

}


export const useZustand = create<ZustandStarterState>()(
    devtools(
        immer(
            persist(
                (_set,_get)=>({

                }),
                {
                    name: "Store",
                    storage: createJSONStorage(() => localStorage),
                }
            )
        ),
        {
            enabled: true,
            name: "Zustand",
        }
    )
)