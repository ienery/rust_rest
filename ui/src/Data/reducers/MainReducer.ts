import { combineReducers } from 'redux';

import { transactsReducer } from '../../modules/transact/Data/reducers/TransactsReducer';
import { routerReducer } from 'react-router-redux'

import {IAppState} from '../Models';
/** 
 * Общий редюсер приложения.
 */
const mainReducer = combineReducers<IAppState>({
    transacts: transactsReducer,
    routing: routerReducer
});

export {mainReducer};