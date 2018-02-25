import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { createStore, applyMiddleware  } from 'redux'
import { Provider } from 'react-redux';
import thunk from 'redux-thunk';

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

export class App implements IApp {
    store: any;

    constructor() {
        this.store = createStore(
            mainReducer,
            applyMiddleware(thunk)
        );
    }

    /** 
     * Рендер компонентов React.
     */
    renderReact() {
        ReactDOM.render(
            <Provider store={this.store}>
                <AppRouter />
            </Provider>, 
        document.getElementById('root'));
    }
}

