import * as React from 'react';
import Loadable from 'react-loadable';

import {
    BrowserRouter as Router,
    Route,
    Link
} from 'react-router-dom';

import {Layout, Menu } from 'antd';

const {Header, Footer, Sider, Content} = Layout;

const Loading = () => {
    return <div>Loading...</div>
  }
  
const Main = Loadable({
    loader: () => import(/* webpackChunkName: "Main" */ './../modules/main/Main'),
    loading: Loading,
});
  
const Transact = Loadable({
    loader: () => import(/* webpackChunkName: "Transact" */ './../modules/transact/Transact'),
    loading: Loading,
});

const TransactCreate = Loadable({
    loader: () => import(/* webpackChunkName: "TransactCreate" */ './../modules/transact/Create/Create'),
    loading: Loading,
});

const TransactsList = Loadable({
    loader: () => import(/* webpackChunkName: "TransactList" */ './../modules/transact/List/TransactsList'),
    loading: Loading,
});

  const TransactDetails = Loadable({
    loader: () => import(/* webpackChunkName: "TransactDetails" */ './../modules/transact/Details/Details'),
    loading: Loading,
  });

class AppRouter extends React.Component {
    render() {
      return (
        <div>
            <Router>
                <Layout className="layout">
                    <Header>
                    <div className="logo" />
                    <Menu
                        theme="dark"
                        mode="horizontal"
                        defaultSelectedKeys={['0']}
                        style={{ lineHeight: '64px' }}
                    >
                        <Menu.Item key="0"><Link to={'/'}>Main</Link></Menu.Item>
                        <Menu.Item key="1"><Link to={'/transact'}>Transact</Link></Menu.Item>
                        <Menu.Item key="2"><Link to={'/transact-create'}>TransactCreate</Link></Menu.Item>
                        <Menu.Item key="3"><Link to={'/transact-details'}>TransactDetails</Link></Menu.Item>
                        <Menu.Item key="4"><Link to={'/transacts'}>TransactsList</Link></Menu.Item>
                    </Menu>
                    </Header>
                    <Content style={{ padding: '0 50px' }}>
                    {/* <Breadcrumb style={{ margin: '16px 0' }}>
                        <Breadcrumb.Item>Home</Breadcrumb.Item>
                        <Breadcrumb.Item>List</Breadcrumb.Item>
                        <Breadcrumb.Item>App</Breadcrumb.Item>
                    </Breadcrumb> */}
                    <div style={{ background: '#fff', padding: 24, minHeight: 280 }}>
                        <Route exact path="/" component={Main}/>
                        <Route path="/transact" component={Transact}/>
                        <Route path="/transact-create" component={TransactCreate}/>
                        <Route path="/transact-details" component={TransactDetails}/>
                        <Route path="/transacts" component={TransactsList}/>
                    </div>
                    </Content>
                    <Footer style={{ textAlign: 'center' }}>
                    Rust Rest 2018 by Ant Design 
                    </Footer>
                </Layout>
            </Router>
        </div>
      )
    }
}

export {AppRouter};
  
