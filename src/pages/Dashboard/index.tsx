import { invoke } from "@tauri-apps/api/core";
import { FC, useEffect,  } from "react";
import { useSearchParams } from "react-router-dom";

const Dashboard: FC = () => {
  const [params] = useSearchParams();
 const cluster = params.get("cluster")
 
  console.log("cluster: ",cluster);
  const list_pods=async (clusterName:string,namespace:string)=>{
    console.log("list_pods params: ",clusterName,namespace);
    
    await invoke("list_pods",{clusterName:clusterName,namespace:namespace}).then((res) => {
        console.log("res: ",res);
    }).catch((err) => {
        console.log("err: ",err);
    })
  }

  useEffect(()=> {
    if (!cluster) return
    list_pods(cluster,"default")
  }, [cluster])
 
  return <div>Dashboard </div>;
};

export default Dashboard;
