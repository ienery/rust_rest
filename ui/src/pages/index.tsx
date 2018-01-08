import * as React from 'react';
import * as ReactDOM from 'react-dom';

import {
    BrowserRouter as Router,
    Route,
    Link
} from 'react-router-dom';

import Bundle from './../components/lazy-load/Bundle';

// BEGIN Компонент транзакций.
import loadTransact from 'bundle-loader?lazy&name=Transact!./../modules/transact/Transact';
import loadTransactCreate from 'bundle-loader?lazy&name=TransactCreate!./../modules/transact/Create/Create';
import loadTransactDetails from 'bundle-loader?lazy&name=TransactDetails!./../modules/transact/Details/Details';

// components load their module for initial visit
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
                <div>
                    <h1>Rust Rest!</h1>
                    <ul>
                            <li><Link to={'/transact'}>Transact</Link></li>
                            <li><Link to={'/transact-create'}>TransactCreate</Link></li>
                            <li><Link to={'/transact-details'}>TransactDetails</Link></li>
                        </ul>
                    <Route path="/transact" component={Transact}/>
                    <Route path="/transact-create" component={TransactCreate}/>
                    <Route path="/transact-details" component={TransactDetails}/>
                </div>
            </Router>
        </div>
      )
    }
}
  

  ReactDOM.render(<App/>, document.getElementById('root'))
  
