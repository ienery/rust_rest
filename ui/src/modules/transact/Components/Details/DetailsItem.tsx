import * as React from 'react';
import { Card, Row, Col } from 'antd';

import {ITransact} from '../../Models';

/**
 * Свойства компонента.
 * 
 * @prop {ITransact} transact Транзакция.
 */
interface IProps {
    transact: ITransact;
}

export const DetailsItem: React.SFC<IProps> = (props): JSX.Element => {
    const {
        parent_transact_id,
        record,
        timestamp,
        transact_id
    } = props.transact;

    return (
        <div>
            <Card title={`DetailsTransactItem: ${transact_id}`}>
                <Card
                    type="inner"
                    title="Record"
                >
                    {/* {Object.keys(record).map((field) => {
                        return (
                            <div>
                                {field}:{record[field]}
                            </div>
                        );
                    })} */}
                    <Row style={{marginBottom: 8}}>
                        <Col span={6}>user_id</Col>
                        <Col span={6}>{record.user_id}</Col>
                    </Row>
                    <Row style={{marginBottom: 8}}>
                        <Col span={6}>point_id</Col>
                        <Col span={6}>{record.point_id}</Col>
                    </Row>
                    <Row style={{marginBottom: 8}}>
                        <Col span={6}>period_year</Col>
                        <Col span={6}>{record.period_year}</Col>
                    </Row>
                    <Row style={{marginBottom: 8}}>
                        <Col span={6}>period_month</Col>
                        <Col span={6}>{record.period_month}</Col>
                    </Row>
                    <Row style={{marginBottom: 8}}>
                        <Col span={6}>readings</Col>
                        <Col span={6}>{record.readings}</Col>
                    </Row>
                    <Row>
                        <Col span={6}>send_date_time</Col>
                        <Col span={6}>{record.send_date_time}</Col>
                    </Row>
                </Card>
                <div
                    style={{
                        fontSize: 14,
                        color: 'rgba(0, 0, 0, 0.85)',
                        marginTop: 16,
                        fontWeight: 400,
                    }}
                >
                    <Row style={{marginBottom: 8}}>
                        <Col span={6}>parent_transact_id</Col>
                        <Col span={18}>{parent_transact_id}</Col>
                    </Row>
                    <Row>
                        <Col span={6}>timestamp</Col>
                        <Col span={18}>{timestamp}</Col>
                    </Row>
                
                </div>
            </Card>
        </div>
    );
}