import * as React from 'react';

import { Row, Col } from 'antd';
import {isObject, transform, isArray, values, keys, reduce} from 'lodash';
/**
 * Компонент меню создание/просмотр транзакции.
 */
class Main extends React.Component {
    static displayName = 'Transact';

    data = {
        a: "1",
        b: {
            b1: "vb1",
            b2: [
                "vb2",
                "vb3",
                "vb4"
            ]
        },
        c: "3",
        d: {
            d1: "vd1",
            d2: "vd2"
        },
        e: [
            "ve1",
            "ve2",
            "ve3"
        ],
        g: {
            g1: {
                h1: "vh1",
                h2: "vh2"
            },
            g2: "vd2",
            g3: {
                m1: "vm1",
                m2: "vm2"
            },
            g4: "vd4",
        },
        n: [
            {o: "v1o"},
            {o: "v2o"},
            {o: "v3o"},
        ],
        p: [
            {
                w1: "v1w",
                z1: {
                    a: "v11a",
                    b: "v12b"
                }
            },
            {
                w1: "v21", 
                z1: {
                    a: "v21a",
                    b: "v22b"
                }
            },
            {
                w1: "v3w",
                z1: {
                    a: "v31a",
                    b: "v32b"
                }
            }
        ]
    }

    

    renderTest() {

        function transformKeyPath(data) {
            return reduce(data, function(result, value, key) {
                if (isArray(value) || isObject(value)) {
                    let resultPath = transformKeyPath(value);

                    let plusKey = resultPath.map(path => key+'.'+path);
                    result.push(...plusKey);
                } else {
                    result.push(key);
                }  

                return result;
            }, []);
        }

        console.debug('renderTest2', this.data);
        let newData = transformKeyPath(this.data);
        console.debug('newData2', newData);
    }

    render() {
        this.renderTest();
        return (
            <div>
                <h1>Rust Rest</h1>
            </div>
        );
    }
}

export default Main;