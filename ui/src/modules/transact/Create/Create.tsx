import * as React from 'react';

import { Row, Col } from '../../../../node_modules/antd/lib/grid/index';

import Form from './Form';
import {IRecord} from '../Models';
import {createRecord} from '../Service';
/**
 * Компонент формы детального просмотра транзакции.
 */
class CreateTransact extends React.Component {
    static displayName = 'CreateTransacts';

    createRecord = (record: IRecord) => {
        createRecord(record);
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
                            createRecord={this.createRecord}
                        />
                    </Col>
                </Row>
            </Row>
        );
    }
}

export default CreateTransact;