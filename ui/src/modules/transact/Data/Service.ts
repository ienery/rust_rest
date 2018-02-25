import axios from 'axios';

import {IRecord, ITransact} from '../Models';
import { resolve } from 'url';

/**
 * Cоздания записи транзакции.
 * 
 * @param {IRecord} record Данные записи транзакции.
 */
export function createTransact (record: IRecord): void {
    console.debug('service transact', record);
    axios.post('/rest/transact/create', {
            record: record
        })
        .then(function (response) {
            console.log(response);
        })
        .catch(function (error) {
            console.log(error);
        });
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
                resolve(response.data);
            })
            .catch(function (error) {
                console.log(error);
                reject(error);
            });
    });
    
};