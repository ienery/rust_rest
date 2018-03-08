import {EStatusResponse} from '../../../Data/Enums';
import {ITransactsState} from '../Data/Models';

/** 
 * Начальное состояние стора. 
 */
export const transactsInitialState: ITransactsState = {
    transacts: null,
    status: EStatusResponse.IDLE,
    block: null
}