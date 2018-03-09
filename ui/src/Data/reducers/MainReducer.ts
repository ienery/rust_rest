import { combineReducers } from 'redux';

import { transactsReducer } from '../../modules/transact/Data/reducers/TransactsReducer';
import { blocksReducer } from '../../modules/Block/Data/Reducers/BlocksReducer';
import { routerReducer } from 'react-router-redux'

import {IAppState} from '../Models';
/** 
 * Общий редюсер приложения.
 */
const mainReducer = combineReducers<IAppState>({
    transacts: transactsReducer,
    blocks: blocksReducer,
    routing: routerReducer
});

export {mainReducer};