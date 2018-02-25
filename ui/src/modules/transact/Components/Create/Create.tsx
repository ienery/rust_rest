import * as React from 'react';

import { Row, Col } from 'antd';

import Form from './Form';
import {IRecord} from '../../Models';
import {createTransact} from '../../Data/Service';

/**
 * Компонент создания транзакции.
 */
class CreateTransact extends React.Component {
    static displayName = 'CreateTransacts';

    createTransact = (record: IRecord) => {
        createTransact(record);
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

export default CreateTransact;