import { EStatusResponse } from '../../../../Data/Enums';
import { readTransacts } from '../Service';

export const GET_TRANSACTS_BEGIN = 'GET_TRANSACTS_BEGIN';
export const GET_TRANSACTS_SUCCESS = 'GET_TRANSACTS_SUCCESS';
export const GET_TRANSACTS_FAILURE = 'GET_TRANSACTS_FAILURE';

/**
 * Экшн запроса транзакций.
 */
export function loadTransactsAction () {
    return function (dispatch) {
        dispatch({
            type: GET_TRANSACTS_BEGIN,
            status: EStatusResponse.LOADING,
        });
        
        readTransacts().then(
            (result) => {
                //console.debug('result', result);
                dispatch(loadTransactsActionSuccess(result))
            },
            (error) => {
                dispatch(loadTransactsActionFailure())
            }
        );
    };
    // return {
    //     type: GET_TRANSACTS_BEGIN,
    //     status: EStatusResponse.LOADING
    // }
}

/**
 * Экшн запроса транзакций - успех.
 */
export function loadTransactsActionSuccess (result) {
    return {
        type: GET_TRANSACTS_SUCCESS,
        status: EStatusResponse.SUCCESS,
        result
    }
}

/**
 * Экшн запроса транзакций - ошибка.
 */
export function loadTransactsActionFailure () {
    return {
        type: GET_TRANSACTS_FAILURE,
        status: EStatusResponse.FAILURE
    }
}