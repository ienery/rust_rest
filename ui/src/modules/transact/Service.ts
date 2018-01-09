import axios from '../../../node_modules/axios/index';

import {IRecord} from './Models';

/**
 * Метод создания записи транзакции.
 * 
 * @param {IRecord} record Данные записи транзакции.
 */
export function createRecord(record: IRecord) {
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