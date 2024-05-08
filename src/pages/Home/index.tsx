import { Button, Col, MenuTheme, Row, Space, Switch, Table, TableProps, Tag } from "antd";
import { FC, useState } from "react";
import { useNavigate } from "react-router-dom";

interface DataType {
    key: string;
    name: string;
    age: number;
    address: string;
    tags: string[];
}

const columns: TableProps<DataType>['columns'] = [
    {
        title: 'Name',
        dataIndex: 'name',
        key: 'name',
        render: (text) => <a>{text}</a>,
    },
    {
        title: 'Age',
        dataIndex: 'age',
        key: 'age',
    },
    {
        title: 'Address',
        dataIndex: 'address',
        key: 'address',
    },
    {
        title: 'Tags',
        key: 'tags',
        dataIndex: 'tags',
        render: (_, { tags }) => (
            <>
                {tags.map((tag) => {
                    let color = tag.length > 5 ? 'geekblue' : 'green';
                    if (tag === 'loser') {
                        color = 'volcano';
                    }
                    return (
                        <Tag color={color} key={tag}>
                            {tag.toUpperCase()}
                        </Tag>
                    );
                })}
            </>
        ),
    },
    {
        title: 'Action',
        key: 'action',
        render: (_, record) => (
            <Space size="middle">
                <a>Invite {record.name}</a>
                <a>Delete</a>
            </Space>
        ),
    },
];

const data: DataType[] = [
    {
        key: '1',
        name: 'John Brown',
        age: 32,
        address: 'New York No. 1 Lake Park',
        tags: ['nice', 'developer'],
    },
    {
        key: '2',
        name: 'Jim Green',
        age: 42,
        address: 'London No. 1 Lake Park',
        tags: ['loser'],
    },
    {
        key: '3',
        name: 'Joe Black',
        age: 32,
        address: 'Sydney No. 1 Lake Park',
        tags: ['cool', 'teacher'],
    },
];
export const Home: FC = () => {
    const [theme, setTheme] = useState<MenuTheme>('dark');
    const changeTheme = (value: boolean) => {
        setTheme(value ? 'dark' : 'light');
    };
    const navigate = useNavigate();
    const redircet = () => {
        navigate('/kubernetes')
    }
    return (
        <>
            <div className="container">
                
                <h1>Kuberntes 列表</h1>
                <div style={{ display: 'flex', justifyContent: 'center' }}>
                    <Table columns={columns} dataSource={data} style={{ width: '80%' }} pagination={false} />
                </div>

            </div>
        </>
    )
}