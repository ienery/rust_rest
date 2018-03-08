import * as React from 'react';
import { push } from 'react-router-redux';
import { connect } from 'react-redux';

import { Row, Col } from 'antd';

import Form from './Form';
import {IRecord} from '../../Models';
import {createTransact} from '../../Data/Service';
import {EPreviousPage} from '../../Enums';


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
        createTransact(record).then(
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
