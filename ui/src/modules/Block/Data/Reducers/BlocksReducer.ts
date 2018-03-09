import { cloneDeep, isUndefined } from 'lodash';

import { IBlockState } from "../Models";
import { EStatusResponse } from "../../../../Data/Enums";
import { blocksInitialState } from "../InitialState";

import {
    GET_BLOCKS_BEGIN,
    GET_BLOCKS_SUCCESS,
    GET_BLOCKS_FAILURE
} from "../Actions/BlocksActions";

/**
 * Редюсер транзакций.
 * 
 * @param {IBlockState} state 
 * @param {} action 
 */
export function blocksReducer (state: IBlockState, action) {
    if (isUndefined(state)) {
        return blocksInitialState;
    }

    let newState = cloneDeep(state);

    if (action.type === GET_BLOCKS_BEGIN) {
        newState = {
            status: EStatusResponse.LOADING,
            blocks: null
        };

        return newState;

    } else if (action.type === GET_BLOCKS_FAILURE) {
        newState.status = EStatusResponse.FAILURE;
        return newState;

    } else if (action.type === GET_BLOCKS_SUCCESS) {
        //console.debug('action', action);
     
        const {blocks} = action.result;
        // результат транзакций в блоке.
        if (blocks) {
            newState.blocks = blocks;
        }
        newState.status = EStatusResponse.SUCCESS;

        return newState;

    } else {
        return state;
    }
}