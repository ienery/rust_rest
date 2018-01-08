import * as React from 'react';

//import { Row, Col } from '../../../node_modules/antd/lib/grid/index';
//import Button from '../../../node_modules/antd/lib/button/index';

const DemoBox = props => <p className={`height-${props.value}`}>{props.children}</p>;

/**
 * Компонент меню создание/просмотр транзакции.
 */
class Transact extends React.Component {
    static displayName = 'Transact';

    render() {
        return (
            <div>
                Transact
            </div>
        );
    }
}

export default Transact;