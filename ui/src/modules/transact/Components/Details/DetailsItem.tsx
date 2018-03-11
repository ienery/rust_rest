import * as React from 'react';
import { Link } from 'react-router-dom';

import { Card, Row, Col } from 'antd';

import {ITransact} from '../../Models';
import {EPreviousPage} from '../../Enums';
/**
 * Свойства компонента.
 * 
 * @prop {ITransact} transact Транзакция.
 * @prop {EPreviousPage} previous Страница - источник перехода.
 * @prop {string} blockId Идентификаторо блока.
 */
interface IProps {
    transact: ITransact;
    previous: EPreviousPage;
    blockId: string;
}

export class DetailsItem extends React.Component<IProps, {}> {
    
    /**
     * Рендер ссылки на список транзакций.
     */
    renderLink = (): JSX.Element => {
        const {previous, blockId} = this.props;

        let urlParams = {
            to: {
                pathname: '/transacts',
                search: ''
            },
            linkLabel: 'To TransactList'
        };
       
        switch (previous) {
            // Переход со страниц создания или просмотра транзакций без блока.
            case EPreviousPage.CREATE:
            case EPreviousPage.NO_BLOCK:
               break;
            default:
                if (blockId) {
                    urlParams.to.search = `?blockId=${blockId}`;
                    urlParams.linkLabel = 'To Block TransactList';
                }
        }

        return (
            <div 
                style={{
                    margin: 8
                }}
            >
                <Link 
                    to={urlParams.to}
                >
                    {urlParams.linkLabel}    
                </Link>
            </div>
        );
            
    }

    render() {
        const {
            parent_transact_id,
            record,
            timestamp,
            transact_id
        } = this.props.transact;

        return (
            <div>
                {this.renderLink()}
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
                        <Row style={{marginBottom: 8}}>
                            <Col span={6}>send_date_timestamp</Col>
                            <Col span={6}>{record.send_date_timestamp}</Col>
                        </Row>
                        <Row>
                            <Col span={6}>period_timestamp</Col>
                            <Col span={6}>{record.period_timestamp}</Col>
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
}