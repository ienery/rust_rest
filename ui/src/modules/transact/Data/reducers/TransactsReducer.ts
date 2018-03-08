import { cloneDeep, isUndefined } from 'lodash';

import { ITransactsState } from "../Models";
import { EStatusResponse } from "../../../../Data/Enums";
import { transactsInitialState } from "../InitialState";

import {
    GET_TRANSACTS_BEGIN,
    GET_TRANSACTS_SUCCESS,
    GET_TRANSACTS_FAILURE
} from "../actions/TransactsActions";

/**
 * Редюсер транзакций.
 * 
 * @param {} state 
 * @param {} action 
 */
export function transactsReducer (state: ITransactsState, action) {
    if (isUndefined(state)) {
        return transactsInitialState;
    }

    let newState = cloneDeep(state);

    if (action.type === GET_TRANSACTS_BEGIN) {
        newState = {
            status: EStatusResponse.LOADING,
            transacts: null,
            block: null
        };

        return newState;

    } else if (action.type === GET_TRANSACTS_FAILURE) {
        newState.status = EStatusResponse.FAILURE;
        return newState;

    } else if (action.type === GET_TRANSACTS_SUCCESS) {
        //console.debug('action', action);
     
        const {block, transacts} = action.result;
        // результат транзакций в блоке.
        if (block) {
            newState.block = block;
        } 
        // результат транзакций вне блока.
        else {
            newState.transacts = transacts;
        }

        //newState.transacts = action.result.body.transacts;
        newState.status = EStatusResponse.SUCCESS;
        //console.debug('newState', newState);
        return newState;

    } else {
        return state;
    }
}