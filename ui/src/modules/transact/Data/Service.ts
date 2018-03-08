import axios from 'axios';

import {IRecord, ITransact} from '../Models';

/**
 * Cоздания записи транзакции.
 * 
 * @param {IRecord} record Данные записи транзакции.
 */
export async function createTransact (record: IRecord) {
    try {
        const response = await axios.post('/rest/transact/create', {
            record: record
        });
        //console.debug('response', response);
        if (response.data.success === true) {
            return response.data.body;
        } 
            
        return null;
    } catch (error) {
        //console.debug(error);
        return false;
    }
};

/**
 * Чтение транзакций не в блоке.
 */
export function readTransacts () {
    //console.debug('service transacts');
    return new Promise((resolve, reject) => {
        axios.post('/rest/transacts/read')
            .then(function (response) {
                //console.log(response);
                //return response.data.body.transacts;
                resolve(response.data.body);
            })
            .catch(function (error) {
                console.log(error);
                reject(error);
            });
    });
};

/**
 * Чтение транзакции.
 * 
 * @param {string} transact_id Идентификатор транзакции.
 */
export async function readTransact (transact_id: string) {
    try {
        const response = await axios.post('/rest/transact/read', {
            transact_id
        });

        //console.debug('response', response);
        if (response.data.success === true) {
            return response.data.body.transact;
        }

        return null;
    } catch (error) {
        return false;
    }
};

/**
 * ИНтерфейс для функции чтения транзакции из блока.
 * 
 * @param {string} transact_id Идентификатор транзакции.
 * @param {string} block_id Идентификатор блока.
 * 
 */
interface ITransactBlock {
    transact_id: string;
    block_id: string;
}

/**
 * Чтение транзакции из блока.
 * 
 * @param {ITransactBlock} params Параметры функции.
 */
export async function readBlockTransact (params: ITransactBlock) {
    try {
        const response = await axios.post('/rest/blocktransact/read', params);

        //console.debug('response', response);
        if (response.data.success === true) {
            return response.data.body.transact;
        }

        return null;
    } catch (error) {
        return false;
    }
};