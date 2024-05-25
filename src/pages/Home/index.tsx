import {  Button, Table, TableProps } from "antd";
import { FC, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { invoke } from "@tauri-apps/api/core";
import { Cluster } from "@/types/cluster";
import init, {greet} from '@mywasm/foo'

export const Home: FC = () => {

    const navigate = useNavigate();
    const redircet = (record:Cluster) => {
        navigate(`/kubernetes?cluster=${record.name}`)
    }
    const columns: TableProps<Cluster>['columns'] = [
        {
            title: '集群名称',
            dataIndex: 'name',
            key: 'name',
            align: 'center',
            render: (text,record) => <a onClick={()=>redircet(record)}>{text}</a>,
        },
        {
            title: '集群地址',
            dataIndex: ['cluster', 'server'],
            key: 'server',
            align: 'center',
        },
    
    ];
    
    const [clusters, setClusters] = useState<Array<Cluster>>([])

    const list_cluster = async () => {
        await invoke("list_clusters").then((res) => {
            setClusters(res as Array<Cluster>)
        })
    }
    useEffect(() => {
        list_cluster()
        init()
    }, [])

    return (
        <>
            <div className="container">
                <div>
                    <Button type="primary" onClick={()=> greet()}>wasm</Button>
                </div>
                <h1>Kuberntes 列表</h1>
                <div style={{ display: 'flex', justifyContent: 'center' }}>
                    <Table rowKey={record => record.name} columns={columns} dataSource={clusters} style={{ width: '80%' }} pagination={false} />
                </div>
            </div>
        </>
    )
}