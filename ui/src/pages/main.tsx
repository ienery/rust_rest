import * as React from 'react';
import * as ReactDOM from 'react-dom';

import {
    BrowserRouter as Router,
    Route,
    Link
} from 'react-router-dom'

import {Header} from '../components/Header';

import { MainPage } from "../components/MainPage";
import { PagePage } from "../components/PagePage";

ReactDOM.hydrate(
    <div>
        <Header
            title="Главная"
        />
        <Router>
            <div>
                <ul>
                    <li><Link to={'/main'}>MainMain</Link></li>
                    <li><Link to={'/main/main'}>MainPage</Link></li>
                    <li><Link to={'/main/page'}>PagePage</Link></li>
                </ul>
                <Route exact path='/main/main' component={ MainPage } />
                <Route path="/main/page" component={ PagePage } />
            </div>
        </Router>
    </div>, 
    document.getElementById('header')
);

ReactDOM.render(
    <div>
        <a href="/page">Page</a> 
    </div>, 
    document.getElementById('root')
);