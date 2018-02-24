import {EStatusResponse} from './Enums';
import {IAppState, ITransactsState} from './Models';

/** 
 * Начальное состояние стора. 
 */
export const transactsInitialState: ITransactsState = {
    data: null,
    status: EStatusResponse.IDLE
}