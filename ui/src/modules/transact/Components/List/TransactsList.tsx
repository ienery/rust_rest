import { includes, isEmpty } from 'lodash';
import * as React from 'react';
import { connect } from 'react-redux';
import { push } from 'react-router-redux';
import  * as queryString from 'query-string';

import { Card, List, Row, Col, Button } from 'antd';

import { readTransacts } from '../../Data/Service';
import { ITransact } from '../../Models';
import { EStatusResponse } from '../../../../Data/Enums';
import { loadTransactsAction, loadBlockTransactsAction } from '../../Data/actions/TransactsActions';
import { TransactListItem } from './TransactListItem';

import { createBlock } from '../../../Block/Data/Service';
import { IBlock } from '../../../Block/Models';

/** 
 * Свойства из connect State.
 * 
 * @prop {boolean} isLoading Загрузка данных.
 * @prop {boolean} hasError Наличие ошибок.
 * @prop {ITransact[]} data Данные.
 * @prop {string} blockId Идентификатор запрашиваемого блока.
 * @prop {IBlock} block Данные в загруженном блоке.
 */
interface IPropsState {
    isLoading: boolean;
    hasError: boolean;
    data: ITransact[];
    blockId: string;
    block: IBlock;
}

/**
 * Свойства из connect Dispatch.
 * 
 * @prop {Function} readTransacts Загрузка транзакций вне блока.
 **@prop {Function} readBlockTransacts Загрузка транзакций из блока.
 * @prop {Function} push Работа с роутом.
 */
interface IPropsDispatch {
    readTransacts: any;
    readBlockTransacts: any;
    push: any;
}

/** Свойства компонента. */
type IProps = IPropsState & IPropsDispatch;

/**
 * Компонент просмотра списка транзакций.
 */
class TransactList extends React.Component<IProps, {}> {
    static displayName = 'TransactList';

    componentDidMount() {
        const {blockId, readTransacts, readBlockTransacts} = this.props;
       
        if (blockId) {
            // Чтение транзакций из блока.
            readBlockTransacts(blockId);
        } else {
            // Чтение транзакций вне блока.
            readTransacts();
        }
        
    }

    /**
     * Обработчик клика создания блока.
     */
    handleClickCreateBlock = () => {
        const {push, readBlockTransacts} = this.props;

        createBlock().then(
            (result) => {
                if (result) {
                    const {block_id} = result.block;

                    this.props.push({
                        pathname: '/transacts',
                        search: `?blockId=${block_id}`
                    });

                    readBlockTransacts(block_id);
                } else {
                    console.error('Error create');
                }
            }
        )
    }

    /**
     * Рендер заголовка списка.
     */
    renderCardTitle (): JSX.Element {
        const {data, block} = this.props;

        return (
            <Row>
                <Col span={6}>
                    {isEmpty(block) ? (
                        <span>TransactsList</span>
                    ) : (
                        <span>Block TransactsList</span>
                    )}
                </Col>
                <Col style={{
                    textAlign: "right"
                }}
                span={18}>
                    {isEmpty(block) ? (
                        !isEmpty(data) && (
                            <Button type="primary" onClick={this.handleClickCreateBlock}>
                                Create Block
                            </Button>
                        )
                    ) : (
                        <div 
                            style={{
                                fontSize: 14,
                                color: 'rgba(0, 0, 0, 0.45)'
                            }}
                        >
                            <div>block_no: {block.block_no}</div>
                            <div>block_id: {block.block_id}</div>
                        </div>
                    )}
                </Col>
            </Row>
        );
    }

    /** 
     * Рендер списка транзакций.
     */
    renderItems (): JSX.Element {
        const {data, push, block} = this.props;

        let block_id  = !isEmpty(block) ? block.block_id : null;

        return (
            <Card 
                title={this.renderCardTitle()}
            >
                {!isEmpty(data) ? (
                    <List
                        itemLayout="horizontal"
                        dataSource={data}
                        renderItem={item => (
                            <TransactListItem item={item} push={push} blockId={block_id} />
                        )}
                    />
                ) : (
                    <div>No data</div>
                )}
            </Card>
            
        );
    }

    render() {
        const {isLoading, hasError} = this.props;

        return (
            <div>
                {isLoading ?
                    <div>Loading... </div> :
                    this.renderItems()
                }
            </div>
        );
    }
}

const mapStateToProps = (state, ownProps) => {
    const {transacts: {status, transacts: dataTransacts, block}} = state;
    const isLoading = includes([EStatusResponse.IDLE, EStatusResponse.LOADING], status);
    const hasError = status === EStatusResponse.FAILURE;

    const {location} = ownProps;
    const {blockId = null} = queryString.parse(location.search);

    let data = null;
    let dataBlock = null;

    // Запрошен блок и есть его данные.
    if (blockId && block) {
        data = block.transacts;
        dataBlock = block;
    } 
    // ПО умолчанию показываются транзакции вне блока.
    else {
        data = dataTransacts;
        dataBlock = null;
    }

    return {
        isLoading,
        hasError,
        data,
        blockId,
        block: dataBlock
    }
}
  
const mapDispatchToProps = (dispatch) => {
    return {
        readTransacts: () => {
            dispatch(loadTransactsAction());
        },
        readBlockTransacts: (blockId: string) => {
            dispatch(loadBlockTransactsAction(blockId));
        },
        push: (path) => {
            dispatch(push(path));
        }
    }
}

const TransactListConnect = connect(
    mapStateToProps,
    mapDispatchToProps
)(TransactList);

export default TransactListConnect;