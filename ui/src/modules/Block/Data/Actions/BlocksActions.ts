import { EStatusResponse } from '../../../../Data/Enums';
import { readBlocks } from '../Service';

export const GET_BLOCKS_BEGIN = 'GET_BLOCKS_BEGIN';
export const GET_BLOCKS_SUCCESS = 'GET_BLOCKS_SUCCESS';
export const GET_BLOCKS_FAILURE = 'GET_BLOCKS_FAILURE';

/**
 * Экшн запроса блоков.
 */
export function loadBlocksAction () {
    return function (dispatch) {
        dispatch({
            type: GET_BLOCKS_BEGIN,
            status: EStatusResponse.LOADING,
        });
        
        readBlocks().then(
            (result) => {
                //console.debug('result', result);
                dispatch(loadBlocksActionSuccess(result))
            },
            (error) => {
                dispatch(loadBlocksActionFailure())
            }
        );
    };
}

/**
 * Экшн запроса блоков - успех.
 */
export function loadBlocksActionSuccess (result) {
    return {
        type: GET_BLOCKS_SUCCESS,
        status: EStatusResponse.SUCCESS,
        result
    }
}

/**
 * Экшн запроса блоков - ошибка.
 */
export function loadBlocksActionFailure () {
    return {
        type: GET_BLOCKS_FAILURE,
        status: EStatusResponse.FAILURE
    }
}
