import {Button, Form, Input} from "@arco-design/web-react";

export const Login = () => {
    return(
        <div>
            <span className={"auth-title"}>
                Login
            </span>
            <Form className="auth-form">
                <Form.Item label={"US"}>
                    <Input/>
                </Form.Item>
                <Form.Item label={"PWD"}>
                    <Input.Password />
                </Form.Item>
                <Form.Item>
                    <Button>
                        Login
                    </Button>
                </Form.Item>
            </Form>
        </div>
    )
}