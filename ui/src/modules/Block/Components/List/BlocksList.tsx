import { includes, isEmpty } from 'lodash';
import * as React from 'react';
import { connect } from 'react-redux';

import { Card, List, Row, Col, Button } from 'antd';

import { IBlock } from '../../Models';
import { EStatusResponse } from '../../../../Data/Enums';
import { loadBlocksAction } from '../../Data/Actions/BlocksActions';

import { BlockListItem } from './BlockListItem';
/** 
 * Свойства из connect State.
 * 
 * @prop {boolean} isLoading Загрузка данных.
 * @prop {boolean} hasError Наличие ошибок.
 * @prop {IBlock[]} data Данные.
 */
interface IPropsState {
    isLoading: boolean;
    hasError: boolean;
    data: IBlock[];
}

/**
 * Свойства из connect Dispatch.
 * 
 * @prop {Function} readBlocks Загрузка всех блоков.
 */
interface IPropsDispatch {
    readBlocks: any;
}

/** Свойства компонента. */
type IProps = IPropsState & IPropsDispatch;

class BlocksList extends React.Component<IProps, {}> {
    static displayName = 'BlocksList';

    componentDidMount() {
        const {readBlocks} = this.props;
  
        readBlocks();
    }

    /**
     * Рендер заголовка списка.
     */
    renderCardTitle (): JSX.Element {
        return (
            <Row>
                <Col span={24}>ALL Blocks</Col>
            </Row>
        );
    }

    /** 
     * Рендер списка блоков.
     */
    renderItems (): JSX.Element {
        const {data} = this.props;

        return (
            <Card 
                title={this.renderCardTitle()}
            >
                {!isEmpty(data) ? (
                    <List
                        itemLayout="horizontal"
                        dataSource={data}
                        renderItem={item => (
                            <BlockListItem item={item} />
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
    const {blocks: {status, blocks: data}} = state;
    const isLoading = includes([EStatusResponse.IDLE, EStatusResponse.LOADING], status);
    const hasError = status === EStatusResponse.FAILURE;

    return {
        isLoading,
        hasError,
        data
    }
}

const mapDispatchToProps = (dispatch) => {
    return {
        readBlocks: () => {
            dispatch(loadBlocksAction());
        }
    }
}

const BlocksListConnect = connect(
    mapStateToProps,
    mapDispatchToProps
)(BlocksList);

export default BlocksListConnect;
