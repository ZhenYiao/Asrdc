import {createBrowserRouter} from "react-router-dom";
import {AuthLayout} from "../view/Auth/Layout.tsx";
import {Login} from "@/view/Auth/Login.tsx";
import HomeLayout from "@/view/Home/Layout.tsx";

export const BasicRouter = createBrowserRouter([
    {
        path: "/",
        element: <h1>Hello Asrdc</h1>
    },
    {
        path: "/auth",
        element: <AuthLayout/>,
        children:[
            {
                path: "login",
                element: <Login/>
            }
        ]
    },
    {
        path: "/home",
        element: <HomeLayout/>,
        children: [

        ]
    }
])