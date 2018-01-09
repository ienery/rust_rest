import * as React from 'react';

import Form from '../../../../node_modules/antd/lib/form/index';
import {FormProps} from '../../../../node_modules/antd/lib/form/form';

import Button from '../../../../node_modules/antd/lib/button/index';
import Input from '../../../../node_modules/antd/lib/input/index';

import {IRecord} from '../Models';

const FormItem = Form.Item;


function hasErrors(fieldsError) {
  return Object.keys(fieldsError).some(field => fieldsError[field]);
}

/**
 * @prop {Function} createRecord Действие при отправке формы.
 * @prop {any} form Заглушка для компопнента.
 */
interface IProps {
    createRecord: (record: IRecord) => void,
    form: any
}

/**
 * Компонент формы создания транзакции.
 */
class CreateTransact extends React.Component<IProps, {}> {
    static displayName = 'CreateTransact';

    componentDidMount() {
        // To disabled submit button at the beginning.
        this.props.form.validateFields();
      }

    handleSubmit = (e) => {
        e.preventDefault();
        this.props.form.validateFields((err, values) => {
          if (!err) {
            //console.log('Received values of form: ', values);
            this.props.createRecord(values);
          }
        });
    }

    render() {
        const { getFieldDecorator } = this.props.form;
        const formItemLayout = {
            labelCol: {
              xs: { span: 24 },
              sm: { span: 10 },
            },
            wrapperCol: {
              xs: { span: 24 },
              sm: { span: 14 },
            },
        };
        const tailFormItemLayout = {
            wrapperCol: {
              xs: {
                span: 24,
                offset: 0,
              },
              sm: {
                span: 14,
                offset: 10,
              },
            },
          };

        return (
            <Form onSubmit={this.handleSubmit} className="transact-form">
                <FormItem
                    {...formItemLayout}
                    label="user_id"
                >
                    {getFieldDecorator('user_id', {
                        rules: [{ required: true, message: 'Please input user_id!' }],
                    })(
                        <Input placeholder="user_id" />
                    )}
                </FormItem>

                <FormItem
                    {...formItemLayout}
                    label="point_id"
                >
                    {getFieldDecorator('point_id', {
                        rules: [{ required: true, message: 'Please input point_id!' }],
                    })(
                        <Input placeholder="point_id" />
                    )}
                </FormItem>

                <FormItem
                    {...formItemLayout}
                    label="period_year"
                >
                    {getFieldDecorator('period_year', {
                        rules: [{ required: true, message: 'Please input period_year!' }],
                    })(
                        <Input placeholder="period_year" />
                    )}
                </FormItem>

                <FormItem
                    {...formItemLayout}
                    label="period_month"
                >
                    {getFieldDecorator('period_month', {
                        rules: [{ required: true, message: 'Please input period_month!' }],
                    })(
                        <Input placeholder="period_month" />
                    )}
                </FormItem>

                <FormItem
                    {...formItemLayout}
                    label="readings"
                >
                    {getFieldDecorator('readings', {
                        rules: [{ required: true, message: 'Please input readings!' }],
                    })(
                        <Input placeholder="readings" />
                    )}
                </FormItem>

                <FormItem
                    {...formItemLayout}
                    label="send_date_time"
                >
                    {getFieldDecorator('send_date_time', {
                        rules: [{ required: true, message: 'Please input send_date_time!' }],
                    })(
                        <Input placeholder="send_date_time" />
                    )}
                </FormItem>

                <FormItem {...tailFormItemLayout}>
                    <Button type="primary" htmlType="submit" className="transact-form-button">
                        Create
                    </Button>
                </FormItem>
            </Form>
        );
    }
}

const WrappedHorizontalLoginForm = Form.create()(CreateTransact);

export default WrappedHorizontalLoginForm;