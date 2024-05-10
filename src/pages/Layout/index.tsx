import React from "react";

import { Layout, Menu, MenuProps, theme } from "antd";
import { Outlet, useNavigate, useSearchParams } from "react-router-dom";
import { HomeOutlined, DiffOutlined, EditOutlined } from "@ant-design/icons";

const { Header, Content, Sider } = Layout;

type MenuItem = Required<MenuProps>["items"][number];
const items: MenuItem[] = [
  {
    label: "首页",
    key: "/kubernetes",
    icon: <HomeOutlined />,
  },
  {
    label: "工作负载",
    key: "/workload",
    icon: <DiffOutlined />,
    children: [
      {
        label: "Deployment",
        key: "/kubernetes/workload/deployment",
        icon: <DiffOutlined />,
      },
      {
        label: "Pod",
        key: "/kubernetes/workload/pod",
        icon: <DiffOutlined />,
      },
    ],
  },
  {
    label: "网络",
    key: "/kuberntes/network",
    icon: <EditOutlined />,
  },
];
const GeekLayout: React.FC = () => {
  const [searchParams] = useSearchParams();
  const clusterName = searchParams.get("cluster");
  console.log("---", clusterName);
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();
  const navigate = useNavigate();
  const onMenuClick: MenuProps["onClick"] = (route) => {
    const path = route.key;
    console.log(path);
    navigate(path);
  };
  return (
    <Layout>
      <Sider
        breakpoint="lg"
        collapsedWidth="0"
        onBreakpoint={(broken) => {
          console.log(broken);
        }}
        onCollapse={(collapsed, type) => {
          console.log(collapsed, type);
        }}
      >
        <div className="demo-logo-vertical" />
        <Menu
          theme="dark"
          mode="inline"
          // defaultSelectedKeys={["4"]}s
          style={{ height: "100%", borderRight: 0 }}
          items={items}
          onClick={onMenuClick}
        />
      </Sider>
      <Layout>
        <Header style={{ padding: 0, background: colorBgContainer }} />
        <Content style={{ margin: "24px 16px 0" }}>
          <div
            style={{
              padding: 24,
              minHeight: 560,
              maxHeight: 560,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            xxxx
            <Outlet />
          </div>
        </Content>
      </Layout>
    </Layout>
  );
};

export default GeekLayout;
