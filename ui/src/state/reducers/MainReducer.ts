import { combineReducers } from 'redux';

import { transactsReducer } from './TransactsReducer';

import {IAppState} from '../Models';
/** 
 * Общий редюсер приложения.
 */
const mainReducer = combineReducers<IAppState>({
    transacts: transactsReducer
});

export {mainReducer};