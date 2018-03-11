import * as React from 'react';
import { push } from 'react-router-redux';
import { connect } from 'react-redux';
import * as moment from 'moment';
import { Row, Col } from 'antd';

import Form from './Form';
import {IRecord} from '../../Models';
import {createTransact} from '../../Data/Service';
import {EPreviousPage} from '../../Enums';

/**
 * Функция для перековертирования данных перед сохраненем.
 * 
 * @param {IRecord} record Запись.
 */
function prepareRecordFields(record: IRecord): IRecord {
    // преобразовать перед сохранением period_timestamp из строки YYYY-MM в ts
    const newPeriodTimestamp = moment.utc(
        `${record.period_year}-${record.period_month}`, 
        'YYYY-MM'
    ).unix().toString();
    const newSendDateTimestamp = moment.utc().unix().toString();

    const newRecord = {
        ...record,
        send_date_timestamp: newSendDateTimestamp,
        period_timestamp: newPeriodTimestamp
    };

    return newRecord;
}

/**
 * Свойства из connect Dispatch.
 * 
 * @prop {Function} push Работа с роутом.
 */
interface IPropsDispatch {
    push: any;
}

/**
 * Компонент создания транзакции.
 */
class CreateTransact extends React.Component<IPropsDispatch, {}> {
    static displayName = 'CreateTransacts';


    createTransact = (record: IRecord) => {
        const newRecord = prepareRecordFields(record);

        createTransact(newRecord).then(
            (result) => {
                if (result) {
                    const {transact_id} = result.transact;
                    let search = `?transactId=${transact_id}&previous=${EPreviousPage.CREATE}`;

                    this.props.push({
                        pathname: '/transact-details',
                        search
                    });
                } else {
                    console.error('Error save');
                }
            }
        );
        
    }

    render() {
        return (
            <Row>
                <Row type="flex" justify="start">
                    <Col span={6}>
                        <h2>Create Transact</h2>
                    </Col>
                </Row>
                <Row type="flex" justify="start">
                    <Col span={6}>
                        <Form 
                            createTransact={this.createTransact}
                        />
                    </Col>
                </Row>
            </Row>
        );
    }
}

const mapDispatchToProps = (dispatch) => {
    return {
        push: (path) => {
            dispatch(push(path));
        }
    }
}

const CreateTransactConnect = connect(
    null,
    mapDispatchToProps
)(CreateTransact);

export default CreateTransactConnect;
