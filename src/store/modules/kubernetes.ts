import { Cluster } from "@/types/cluster";
import { PayloadAction, createSlice } from "@reduxjs/toolkit";


const k8sStore = createSlice({
    name: 'k8s',
    initialState: {
        clusters: [] as Array<Cluster>
    },
    reducers: {
        setClusters(state, action:PayloadAction<Array<Cluster>>) {
            state.clusters = action.payload
        }
    }
})

const { setClusters } = k8sStore.actions


export {setClusters}
export default k8sStore.reducer;