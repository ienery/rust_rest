import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { createStore, applyMiddleware  } from 'redux'
import { Provider } from 'react-redux';
import thunk from 'redux-thunk';
import { routerMiddleware } from 'react-router-redux'
import createHistory from 'history/createBrowserHistory'

import { AppRouter } from './AppRouter';
import { mainReducer } from '../Data/reducers/MainReducer';

//import {initialState} from '../state/InitialState';

/** 
 * Стор для редакса.
 */
interface StoreEnhancerState {};

/** 
 * Свойства приложения.
 * 
 * @prop {any} store Стор редакса.
 */
interface IApp {
    store: any;
}

// Create a history of your choosing (we're using a browser history in this case)
const history = createHistory();

// Build the middleware for intercepting and dispatching navigation actions
const routeMiddleware = routerMiddleware(history);

export class App implements IApp {
    store: any;
    middleware: any;
    
    constructor() {
        

        this.store = createStore(
            mainReducer,
            applyMiddleware(thunk, routeMiddleware)
        );
    }

    /** 
     * Рендер компонентов React.
     */
    renderReact() {
        ReactDOM.render(
            <Provider store={this.store}>
                <AppRouter
                    history={history}
                />
            </Provider>, 
        document.getElementById('root'));
    }
}

