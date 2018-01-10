import * as React from 'react';
import * as ReactDOM from 'react-dom';

import Loadable from 'react-loadable';

import {
    BrowserRouter as Router,
    Route,
    Link
} from 'react-router-dom';

import Layout from '../../node_modules/antd/lib/layout/index';
import Menu from '../../node_modules/antd/lib/menu/index';
//import Breadcrumb from '../../node_modules/antd/lib/breadcrumb/index';

const { Header, Footer, Sider, Content } = Layout;

const Loading = () => {
    return <div>Loading...</div>
  }
  
  const Main = Loadable({
    loader: () => import('./../modules/main/Main'),
    loading: Loading,
  });
  
  const Transact = Loadable({
    loader: () => import('./../modules/transact/Transact'),
    loading: Loading,
  });

  const TransactCreate = Loadable({
    loader: () => import('./../modules/transact/Create/Create'),
    loading: Loading,
  });

  const TransactDetails = Loadable({
    loader: () => import('./../modules/transact/Details/Details'),
    loading: Loading,
  });

  /*
import Bundle from './../components/lazy-load/Bundle';

import loadMain from 'bundle-loader?lazy&name=Main!./../modules/main/Main';
// BEGIN Компонент транзакций.
import loadTransact from 'bundle-loader?lazy&name=Transact!./../modules/transact/Transact';
import loadTransactCreate from 'bundle-loader?lazy&name=TransactCreate!./../modules/transact/Create/Create';
import loadTransactDetails from 'bundle-loader?lazy&name=TransactDetails!./../modules/transact/Details/Details';

// components load their module for initial visit
const Main = (props) => (
    <Bundle load={loadMain}>
        {(Main) => <Main {...props}/>}
    </Bundle>
);

const Transact = (props) => (
    <Bundle load={loadTransact}>
        {(Transact) => <Transact {...props}/>}
    </Bundle>
);
  
const TransactCreate = (props) => (
    <Bundle load={loadTransactCreate}>
        {(TransactCreate) => <TransactCreate {...props}/>}
    </Bundle>
);

const TransactDetails = (props) => (
    <Bundle load={loadTransactDetails}>
        {(TransactDetails) => <TransactDetails {...props}/>}
    </Bundle>
);
*/
// END Компонент транзакций.

class App extends React.Component {
    componentDidMount() {
      // preloads the rest
      //loadAbout(() => {})
      //loadDashboard(() => {})
    }
  
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
                    </div>
                    </Content>
                    <Footer style={{ textAlign: 'center' }}>
                    Rust Rest by Ant Design
                    </Footer>
                </Layout>
            </Router>
        </div>
      )
    }
}
  
ReactDOM.render(<App/>, document.getElementById('root'))
  
