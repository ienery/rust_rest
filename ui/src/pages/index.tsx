import * as React from 'react';
import * as ReactDOM from 'react-dom';

import {
    BrowserRouter as Router,
    Route,
    Link
} from 'react-router-dom';

import Bundle from './../components/lazy-load/Bundle';

import loadDashboard from 'bundle-loader?lazy&name=Dashboard!./../components/Dashboard';
import loadAbout from 'bundle-loader?lazy&name=About!./../components/About';

// components load their module for initial visit
const About = (props) => (
    <Bundle load={loadAbout}>
      {(About) => <About {...props}/>}
    </Bundle>
  )
  
  const Dashboard = (props) => (
    <Bundle load={loadDashboard}>
      {(Dashboard) => <Dashboard {...props}/>}
    </Bundle>
  )
  
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
                    <li><Link to={'/about'}>About</Link></li>
                    <li><Link to={'/dashboard'}>Dashboard</Link></li>
                </ul>
               <Route path="/about" component={About}/>
               <Route path="/dashboard" component={Dashboard}/>
           </div>
        </Router>
        </div>
      )
    }
  }
  

  ReactDOM.render(<App/>, document.getElementById('root'))
  
